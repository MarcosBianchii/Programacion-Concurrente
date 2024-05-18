//! Implementation of a RwLock using Condvars and a Mutex.

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{Condvar, Mutex},
};

#[derive(Default)]
struct Entities {
    readers: usize,
    waiters: usize,
    writing: bool,
}

struct RwLock<T> {
    lock: Mutex<Entities>,
    data: UnsafeCell<T>,
    rcvar: Condvar,
    wcvar: Condvar,
}

impl<T> RwLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: Mutex::new(Entities::default()),
            data: UnsafeCell::new(data),
            rcvar: Condvar::new(),
            wcvar: Condvar::new(),
        }
    }

    pub fn read(&self) -> ReadGuard<'_, T> {
        let entities = self.lock.lock().unwrap();

        let mut entities = self
            .rcvar
            .wait_while(entities, |ent| ent.writing || ent.waiters > 0)
            .unwrap();

        entities.readers += 1;
        ReadGuard::new(self)
    }

    pub fn write(&self) -> WriteGuard<'_, T> {
        let mut entities = self.lock.lock().unwrap();
        entities.waiters += 1;

        let mut entities = self
            .wcvar
            .wait_while(entities, |ent| ent.writing || ent.readers > 0)
            .unwrap();

        entities.writing = true;
        entities.waiters -= 1;
        WriteGuard::new(self)
    }
}

struct ReadGuard<'a, T> {
    lock: &'a Mutex<Entities>,
    data: NonNull<T>,
    wcvar: &'a Condvar,
}

impl<'a, T> ReadGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> Self {
        Self {
            lock: &lock.lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            wcvar: &lock.wcvar,
        }
    }
}

impl<'a, T> Deref for ReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<'a, T> Drop for ReadGuard<'_, T> {
    fn drop(&mut self) {
        let mut entities = self.lock.lock().unwrap();
        entities.readers -= 1;

        if entities.readers == 0 {
            self.wcvar.notify_one();
        }
    }
}

struct WriteGuard<'a, T> {
    lock: &'a Mutex<Entities>,
    data: NonNull<T>,
    rcvar: &'a Condvar,
    wcvar: &'a Condvar,
}

impl<'a, T> WriteGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> Self {
        Self {
            lock: &lock.lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            rcvar: &lock.rcvar,
            wcvar: &lock.wcvar,
        }
    }
}

impl<'a, T> Deref for WriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<'a, T> DerefMut for WriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl<'a, T> Drop for WriteGuard<'_, T> {
    fn drop(&mut self) {
        let mut entities = self.lock.lock().unwrap();
        entities.writing = false;

        match entities.waiters {
            0 => self.rcvar.notify_all(),
            _ => self.wcvar.notify_one(),
        }
    }
}
