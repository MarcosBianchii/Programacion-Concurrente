use crate::{
    concurrent_deque::{ConDeque, Stolen},
    semaphore::Semaphore,
};

// Helper type for representing a
// closure to execute by a thread.
type Task = Box<dyn FnOnce() + Send>;

/// The common data every worker has.
pub struct SharedData {
    tasks: Vec<ConDeque<Task>>,
    num_tasks: Semaphore,
    running: Semaphore,
    nworkers: usize,
}

impl SharedData {
    pub fn new(nworkers: usize) -> Self {
        Self {
            tasks: (0..nworkers).map(|_| ConDeque::new()).collect(),
            num_tasks: Semaphore::new(0),
            running: Semaphore::new(0),
            nworkers,
        }
    }

    fn next_task(&self, id: usize) -> Option<Task> {
        self.num_tasks.acquire();

        // Check own deque.
        if let Some(task) = self.tasks[id].pop() {
            return Some(task);
        }

        // Check other deques.
        for i in 1..self.nworkers {
            let steal_id = (id + i) % self.nworkers;

            loop {
                match self.tasks[steal_id].steal() {
                    Stolen::Empty => break,
                    Stolen::Abort => {}
                    Stolen::Element(task) => return Some(task),
                }
            }
        }

        None
    }

    pub fn run(&self, id: usize) {
        while let Some(task) = self.next_task(id) {
            self.running.release();
            task();
            self.running.acquire();
        }
    }

    pub fn ship_task(&self, id: usize, task: Task) {
        self.tasks[id].push(task);
        self.num_tasks.release();
    }

    pub fn join(&self) {
        self.num_tasks.wait_till_empty();
        self.running.wait_till_empty();
    }

    pub fn kill(&self) {
        // Run workers with no tasks.
        for _ in 0..self.nworkers {
            self.num_tasks.release();
        }
    }
}
