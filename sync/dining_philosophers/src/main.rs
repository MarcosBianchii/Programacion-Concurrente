use rand::Rng;
use std::{iter, thread, time::Duration};
use std_semaphore::Semaphore;

const NPHILOSOPHERS: i32 = 5;

/// Each philosopher has his own stick and depending on the initialization
/// method, will also take the corresponding philosopher's stick.
struct Philosopher {
    stick: Semaphore,
    dominant: usize,
}

impl Philosopher {
    fn euclidean_mod(n: i32, modulo: i32) -> usize {
        ((n % modulo + modulo) % modulo) as usize
    }

    fn new(n: i32) -> Self {
        Self {
            stick: Semaphore::new(1),
            dominant: Self::euclidean_mod(n, NPHILOSOPHERS),
        }
    }

    pub fn left(id: i32) -> Self {
        Self::new(id - 1)
    }

    pub fn right(id: i32) -> Self {
        Self::new(id + 1)
    }

    pub fn acquire_stick(&self) {
        self.stick.acquire();
    }

    pub fn release_stick(&self) {
        self.stick.release();
    }

    pub fn dominant_hand_id(&self) -> usize {
        self.dominant
    }
}

fn main() {
    // Instance N-1 right-handed and one left-handed.
    let table: &[_] = iter::once(Philosopher::left(0))
        .chain((1..NPHILOSOPHERS).map(Philosopher::right))
        .collect::<Vec<_>>()
        .leak();

    (0..NPHILOSOPHERS)
        .map(|id| {
            thread::spawn(move || loop {
                let philosopher = &table[id as usize];

                // Wait and think.
                println!("[{id}] Thinking...");
                let mut randomizer = rand::thread_rng();
                thread::sleep(Duration::from_secs(randomizer.gen_range(1..=30)));

                // Acquire both sticks.
                philosopher.acquire_stick();
                println!("[{id}] Acquired first stick");

                let other_stick_id = philosopher.dominant_hand_id();
                table[other_stick_id].acquire_stick();
                println!("[{id}] Acquired second stick");

                // Eat.
                println!("[{id}] Eating...");
                thread::sleep(Duration::from_secs(10));

                // Returns sticks.
                philosopher.release_stick();
                table[other_stick_id].release_stick();
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|philosopher| philosopher.join().unwrap());
}
