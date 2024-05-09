use rand::Rng;
use std::{sync::Arc, thread, time::Duration};
use std_semaphore::Semaphore;

struct Table {
    sticks: Vec<Semaphore>,
}

impl Table {
    pub fn new(n: usize) -> Self {
        Self {
            sticks: (0..n).map(|_| Semaphore::new(1)).collect(),
        }
    }
}

/// Represents the philosopher's preference when grabbing the
/// sticks, `Pref::Their` means they will grab their own stick first
/// and `Pref::Other` means they will grab the next philosopher's
/// stick first.
enum Pref {
    Their,
    Other,
}

struct Philosopher {
    id: usize,
    pref: Pref,
    name: String,
}

impl Philosopher {
    pub fn new(id: usize, pref: Pref, name: &str) -> Self {
        Self {
            id,
            pref,
            name: name.to_string(),
        }
    }

    fn log(&self, action: &str) {
        let Self { id, name, .. } = self;
        println!("{id}: [{name}] \t{action}");
    }

    pub fn think(&self) {
        self.log("Thinking...");
        let mut randomizer = rand::thread_rng();
        thread::sleep(Duration::from_secs(randomizer.gen_range(1..=30)));
    }

    pub fn eat(&self) {
        self.log("Eating...");
        thread::sleep(Duration::from_secs(10));
    }

    fn sticks<'a>(&self, table: &'a Table) -> (&'a Semaphore, &'a Semaphore) {
        let &Self { id, .. } = self;
        let mine = &table.sticks[id];
        let other = &table.sticks[(id + 1) % table.sticks.len()];
        (mine, other)
    }

    pub fn grab_sticks(&self, table: &Table) {
        let (mine, other) = self.sticks(table);

        let (first, second) = match self.pref {
            Pref::Their => (mine, other),
            Pref::Other => (other, mine),
        };

        first.acquire();
        self.log("Acquired first stick");

        second.acquire();
        self.log("Acquired second stick");
    }

    pub fn return_sticks(&self, table: &Table) {
        let (mine, other) = self.sticks(table);
        mine.release();
        other.release();
    }
}

fn main() {
    let philosophers = [
        Philosopher::new(0, Pref::Their, "Plato"),
        Philosopher::new(1, Pref::Their, "Confucius"),
        Philosopher::new(2, Pref::Their, "Socrates"),
        Philosopher::new(3, Pref::Their, "Voltaire"),
        Philosopher::new(4, Pref::Other, "Descartes"),
    ];

    let table = Arc::new(Table::new(philosophers.len()));

    philosophers
        .into_iter()
        .map(|philosopher| {
            let table = table.clone();
            thread::spawn(move || loop {
                philosopher.think();
                philosopher.grab_sticks(&table);
                philosopher.eat();
                philosopher.return_sticks(&table);
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|thread| thread.join().unwrap());
}
