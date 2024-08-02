use super::shared_data::SharedData;
use std::{
    sync::Arc,
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<JoinHandle<()>>,
    shared_data: Arc<SharedData>,
    next_worker: usize,
}

impl ThreadPool {
    /// Instanciates a new thread pool with `nworkers` workers.
    pub fn new(nworkers: usize) -> Self {
        let mut workers = Vec::with_capacity(nworkers);
        let shared_data = Arc::new(SharedData::new(nworkers));

        for id in 0..nworkers {
            let shared = shared_data.clone();
            let handle = thread::spawn(move || shared.run(id));
            workers.push(handle);
        }

        Self {
            shared_data,
            workers,
            next_worker: 0,
        }
    }

    /// Delegates the given closure to some worker to execute it.
    pub fn execute<F: FnOnce() + Send + 'static>(&mut self, f: F) {
        let id = self.next_worker;
        self.next_worker = (id + 1) % self.workers.len();
        self.shared_data.ship_task(id, Box::new(f));
    }

    /// Waits for all workers to finish their tasks.
    pub fn join(&self) {
        self.shared_data.join();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.shared_data.join();
        self.shared_data.kill();

        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}
