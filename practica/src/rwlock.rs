//! Implementation of a RwLock using Condvars and a Mutex.

use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    ptr::NonNull,
    sync::{Condvar, Mutex},
};

struct PoisonError<T> {
    guard: T,
}

impl<T> PoisonError<T> {}

type LockResult<T> = Result<T, PoisonError<T>>;

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

    pub fn read(&self) -> LockResult<RwLockReadGuard<T>> {
        let entities = self.lock.lock().unwrap();

        let mut entities = self
            .rcvar
            .wait_while(entities, |ent| ent.writing || ent.waiters > 0)
            .unwrap();

        entities.readers += 1;
        RwLockReadGuard::new(self)
    }

    pub fn write(&self) -> LockResult<RwLockWriteGuard<T>> {
        let mut entities = self.lock.lock().unwrap();
        entities.waiters += 1;

        let mut entities = self
            .wcvar
            .wait_while(entities, |ent| ent.writing || ent.readers > 0)
            .unwrap();

        entities.writing = true;
        entities.waiters -= 1;
        RwLockWriteGuard::new(self)
    }
}

struct RwLockReadGuard<'a, T> {
    lock: &'a Mutex<Entities>,
    data: NonNull<T>,
    wcvar: &'a Condvar,
}

impl<'a, T> RwLockReadGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> LockResult<Self> {
        Ok(Self {
            lock: &lock.lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            wcvar: &lock.wcvar,
        })
    }
}

impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<T> Drop for RwLockReadGuard<'_, T> {
    fn drop(&mut self) {
        let mut entities = self.lock.lock().unwrap();
        entities.readers -= 1;

        if entities.readers == 0 {
            self.wcvar.notify_one();
        }
    }
}

struct RwLockWriteGuard<'a, T> {
    lock: &'a Mutex<Entities>,
    data: NonNull<T>,
    rcvar: &'a Condvar,
    wcvar: &'a Condvar,
}

impl<'a, T> RwLockWriteGuard<'a, T> {
    fn new(lock: &'a RwLock<T>) -> LockResult<Self> {
        Ok(Self {
            lock: &lock.lock,
            data: unsafe { NonNull::new_unchecked(lock.data.get()) },
            rcvar: &lock.rcvar,
            wcvar: &lock.wcvar,
        })
    }
}

impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { self.data.as_ref() }
    }
}

impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.data.as_mut() }
    }
}

impl<T> Drop for RwLockWriteGuard<'_, T> {
    fn drop(&mut self) {
        let mut entities = self.lock.lock().unwrap();
        entities.writing = false;

        match entities.waiters {
            0 => self.rcvar.notify_all(),
            _ => self.wcvar.notify_one(),
        }
    }
}
