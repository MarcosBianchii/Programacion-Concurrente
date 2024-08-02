use std::sync::{Condvar, Mutex};

pub struct Semaphore {
    lock: Mutex<usize>,
    cvar: Condvar,
    empty: Condvar,
}

impl Semaphore {
    /// Creates a new `Semaphore` with a given initial resource count.
    pub fn new(count: usize) -> Self {
        Self {
            lock: Mutex::new(count),
            cvar: Condvar::new(),
            empty: Condvar::new(),
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

        if *count == 0 {
            self.empty.notify_all();
        }
    }

    /// Returns a resource from this semahore.
    ///
    /// Will notify an arbitrary thread waiting for this resource.
    pub fn release(&self) {
        *self.lock.lock().unwrap() += 1;
        self.cvar.notify_one();
    }

    /// Waits till there is no resource left.
    pub fn wait_till_empty(&self) {
        let count_guard = self.lock.lock().unwrap();

        let _count = self
            .empty
            .wait_while(count_guard, |count| *count > 0)
            .unwrap();
    }
}
