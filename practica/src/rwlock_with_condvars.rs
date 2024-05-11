//! Implementation of a RwLock using Condvars and a Mutex.

use std::sync::{Condvar, Mutex};

#[derive(Default)]
struct RwLock {
    mutex: Mutex<()>,
    read: Condvar,
    write: Condvar,
    readers: usize,
    writers: usize,
}

impl RwLock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn read(&mut self) {
        let guard = self.mutex.lock().unwrap();
        let _a = self.read.wait_while(guard, |_| self.writers > 0).unwrap();
        self.readers += 1;
    }

    pub fn write(&mut self) {
        let guard = self.mutex.lock().unwrap();
        let _a = self
            .write
            .wait_while(guard, |_| self.writers > 0 || self.readers > 0)
            .unwrap();

        self.writers += 1;
    }

    pub fn drop_read(&mut self) {
        let _guard = self.mutex.lock().unwrap();
        self.readers -= 1;
        self.read.notify_one();
    }

    pub fn drop_write(&mut self) {
        let _guard = self.mutex.lock().unwrap();
        self.writers -= 1;
        self.write.notify_one();
    }
}

fn main() {
    let mut rwlock = RwLock::new();
    rwlock.read();
    println!("0");

    rwlock.write();
    println!("1");

    // println!("2");
    // println!("3");
    // println!("4");
}
