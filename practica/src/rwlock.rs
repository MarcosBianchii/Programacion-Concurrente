//! Implementation of a RwLock using Condvars and a Mutex.

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{Condvar, Mutex},
};

struct RwLock<T> {
    // .0: readers  .1: writers
    lock: Mutex<(usize, usize)>,
    data: UnsafeCell<T>,
    rcvar: Condvar,
    wcvar: Condvar,
}

impl<T> RwLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: Mutex::new((0, 0)),
            data: UnsafeCell::new(data),
            rcvar: Condvar::new(),
            wcvar: Condvar::new(),
        }
    }

    pub fn read(&self) -> ReadGuard<'_, T> {
        let mutex_guard = self.lock.lock().unwrap();

        let mut guard = self
            .rcvar
            .wait_while(mutex_guard, |(_, ws)| *ws > 0)
            .unwrap();

        guard.0 += 1;
        ReadGuard::new(self)
    }

    pub fn write(&self) -> WriteGuard<'_, T> {
        let mutex_guard = self.lock.lock().unwrap();

        let mut guard = self
            .wcvar
            .wait_while(mutex_guard, |rsws| *rsws != (0, 0))
            .unwrap();

        guard.1 += 1;
        WriteGuard::new(self)
    }
}

struct ReadGuard<'a, T> {
    lock: &'a RwLock<T>,
    data: NonNull<T>,
}

impl<'a, T> ReadGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> Self {
        Self {
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            lock,
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
        let mut guard = self.lock.lock.lock().unwrap();
        guard.0 -= 1;

        if guard.0 == 0 {
            self.lock.wcvar.notify_one();
        }
    }
}

struct WriteGuard<'a, T> {
    lock: &'a RwLock<T>,
    data: NonNull<T>,
}

impl<'a, T> WriteGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> Self {
        Self {
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            lock,
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
        let mut guard = self.lock.lock.lock().unwrap();
        guard.1 -= 1;

        match guard.1 {
            0 => self.lock.rcvar.notify_all(),
            _ => self.lock.wcvar.notify_one(),
        }
    }
}
