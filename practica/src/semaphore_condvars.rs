//! Implementation of a Semaphore using Condvar and a Mutex.

use std::sync::{Condvar, Mutex};

#[derive(Default)]
struct Semaphore {
    mutex: Mutex<()>,
    waiting_release: Condvar,
    counter: usize,
}

impl Semaphore {
    pub fn new(state: usize) -> Self {
        Self {
            mutex: Mutex::new(()),
            waiting_release: Condvar::new(),
            counter: state,
        }
    }

    pub fn acquire(&mut self) {
        let guard = self.mutex.lock().unwrap();

        let _a = self
            .waiting_release
            .wait_while(guard, |_| self.counter == 0)
            .unwrap();

        self.counter -= 1;
    }

    pub fn release(&mut self) {
        let mut _guard = self.mutex.lock().unwrap();
        self.counter += 1;
        self.waiting_release.notify_one();
    }
}

fn main() {
    let mut semaphore = Semaphore::new(2);
}
