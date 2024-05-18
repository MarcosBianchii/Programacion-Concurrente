//! Implementation of a Semaphore using Condvar and a Mutex.

use std::sync::{Condvar, Mutex};

struct Semaphore {
    lock: Mutex<usize>,
    cvar: Condvar,
}

impl Semaphore {
    /// Creates a new `Semaphore` with a given initial resource count.
    pub fn new(count: usize) -> Self {
        Self {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
        }
    }

    /// Blocks the calling thread until a resource is available.
    pub fn acquire(&self) {
        let count_guard = self.lock.lock().unwrap();

        let mut count = self
            .cvar
            .wait_while(count_guard, |count| *count == 0)
            .unwrap();

        *count -= 1;
    }

    /// Returns a resource from this semahore.
    ///
    /// Will notify an arbitrary thread waiting for this resource.
    pub fn release(&self) {
        *self.lock.lock().unwrap() += 1;
        self.cvar.notify_one();
    }

    /// Acquires a resource and reutrns a `RAII` guard of `Semaphore`.
    pub fn access(&self) -> SemaphoreGuard {
        SemaphoreGuard::new(self)
    }
}

struct SemaphoreGuard<'a> {
    sem: &'a Semaphore,
}

impl<'a> SemaphoreGuard<'a> {
    fn new(sem: &'a Semaphore) -> Self {
        sem.acquire();
        Self { sem }
    }
}

impl<'a> Drop for SemaphoreGuard<'_> {
    fn drop(&mut self) {
        self.sem.release();
    }
}
