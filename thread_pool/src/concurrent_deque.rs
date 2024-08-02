//! Implementation of the Chevy-Lev deque, from the paper: https://www.dre.vanderbilt.edu/~schmidt/PDF/work-stealing-dequeue.pdf

use std::{
    alloc::{self, Layout},
    mem,
    sync::atomic::{fence, AtomicIsize, AtomicPtr, Ordering::*},
};

const MIN_LOG_SIZE: usize = 7;

pub struct ConDeque<T> {
    bot: AtomicIsize,
    top: AtomicIsize,
    active_array: AtomicPtr<CircularArray<T>>,
}

unsafe impl<T> Sync for ConDeque<T> {}
unsafe impl<T> Send for ConDeque<T> {}

#[derive(Debug)]
pub enum Stolen<T> {
    Empty,
    Abort,
    Element(T),
}

impl<T> ConDeque<T> {
    pub fn new() -> Self {
        let active_array = Box::new(CircularArray::new(MIN_LOG_SIZE));
        Self {
            bot: AtomicIsize::new(0),
            top: AtomicIsize::new(0),
            active_array: AtomicPtr::new(Box::into_raw(active_array)),
        }
    }

    pub fn push(&self, x: T) {
        let bot = self.bot.load(Relaxed);
        let top = self.top.load(Acquire);
        let mut arr = self.active_array.load(Relaxed);
        let size = bot.wrapping_sub(top);

        unsafe {
            if size >= (*arr).size() - 1 {
                arr = Box::into_raw(Box::from_raw(arr).grow(bot, top));
                self.active_array.store(arr, Relaxed);
            }

            (*arr).put(bot, x);
        }

        fence(Release);
        self.bot.store(bot.wrapping_add(1), Relaxed);
    }

    pub fn pop(&self) -> Option<T> {
        let bot = self.bot.load(Relaxed);
        let bot = bot.wrapping_sub(1);
        self.bot.store(bot, Relaxed);

        let top = self.top.load(Relaxed);
        let size = bot.wrapping_sub(top);

        if size < 0 {
            self.bot.store(top, SeqCst);
            return None;
        }

        let arr = self.active_array.load(Relaxed);
        let data = unsafe { (*arr).get(bot) };

        if size > 0 {
            return Some(data);
        }

        let cmpe = self
            .top
            .compare_exchange(top, top.wrapping_add(1), SeqCst, SeqCst);

        self.bot.store(top.wrapping_add(1), Relaxed);
        if cmpe.is_ok() {
            Some(data)
        } else {
            mem::forget(data);
            None
        }
    }

    pub fn steal(&self) -> Stolen<T> {
        let top = self.top.load(Acquire);
        fence(SeqCst);
        let bot = self.bot.load(Acquire);
        let size = bot.wrapping_sub(top);

        if size <= 0 {
            return Stolen::Empty;
        }

        let arr = self.active_array.load(Acquire);
        let data = unsafe { (*arr).get(top) };

        let cmpe = self
            .top
            .compare_exchange(top, top.wrapping_add(1), SeqCst, SeqCst);

        if cmpe.is_ok() {
            Stolen::Element(data)
        } else {
            mem::forget(data);
            Stolen::Abort
        }
    }
}

impl<T> Drop for ConDeque<T> {
    fn drop(&mut self) {
        let arr = self.active_array.load(Relaxed);
        let bot = self.bot.load(Relaxed);
        let top = self.top.load(Relaxed);

        let mut i = top;
        while i != bot {
            unsafe { (*arr).get(i) };
            i = i.wrapping_add(1);
        }

        unsafe { drop(Box::from_raw(arr)) };
    }
}

struct CircularArray<T> {
    ptr: *mut T,
    log_size: usize,
    prev: Option<Box<Self>>,
}

impl<T> CircularArray<T> {
    fn new(log_size: usize) -> Self {
        let layout = Layout::array::<T>(1 << log_size).unwrap();
        let ptr = unsafe { alloc::alloc(layout) as *mut T };

        Self {
            log_size,
            ptr,
            prev: None,
        }
    }

    fn size(&self) -> isize {
        (1 << self.log_size) as isize
    }

    unsafe fn elem(&self, i: isize) -> *mut T {
        let mask = (1 << self.log_size) - 1;
        self.ptr.offset(i & mask)
    }

    fn get(&self, i: isize) -> T {
        unsafe { self.elem(i).read() }
    }

    fn put(&self, i: isize, x: T) {
        unsafe { self.elem(i).write(x) };
    }

    fn grow(self: Box<Self>, bot: isize, top: isize) -> Box<Self> {
        let mut new_arr = Self::new(self.log_size + 1);

        let mut i = top;
        while i != bot {
            new_arr.put(i, self.get(i));
            i = i.wrapping_add(1);
        }

        new_arr.prev = Some(self);
        Box::new(new_arr)
    }
}

impl<T> Drop for CircularArray<T> {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::array::<T>(1 << self.log_size).unwrap();
            alloc::dealloc(self.ptr as *mut u8, layout);
        }
    }
}
