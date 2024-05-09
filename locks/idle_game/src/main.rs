use rand::Rng;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    thread,
    time::Duration,
};

const NEXTRACTORS: usize = 3;
const NCONVERTERS: usize = 3;
const NCONSUMERS: usize = 3;

#[derive(Hash, PartialEq, Eq, Debug)]
enum Resource {
    Wood,
    Plastic,
    Metal,
}

impl Resource {
    const WOOD_COST: usize = 5;
    const PLASTIC_COST: usize = 10;
    const METAL_COST: usize = 15;

    pub fn cost(&self) -> usize {
        match *self {
            Self::Wood => Self::WOOD_COST,
            Self::Plastic => Self::PLASTIC_COST,
            Self::Metal => Self::METAL_COST,
        }
    }

    pub fn purchasing_criteria(amount: usize) -> Option<Self> {
        Some(match amount {
            Self::METAL_COST.. => Self::Metal,
            Self::PLASTIC_COST.. => Self::Plastic,
            Self::WOOD_COST.. => Self::Wood,
            _ => return None,
        })
    }
}

fn main() {
    let gold = Arc::new(RwLock::new(0));
    let resources = Arc::new(RwLock::new(HashMap::<_, usize>::new()));

    // Create N extractors.
    for _ in 0..NEXTRACTORS {
        let gold = Arc::clone(&gold);
        thread::spawn(move || loop {
            let generated = rand::thread_rng().gen_range(1..=10);
            *gold.write().unwrap() += generated;
            thread::sleep(Duration::from_millis(500));
        });
    }

    // Create N converters.
    for _ in 0..NCONVERTERS {
        let gold = Arc::clone(&gold);
        let resources = Arc::clone(&resources);
        thread::spawn(move || loop {
            let mut gold = gold.write().unwrap();

            if let Some(resource) = Resource::purchasing_criteria(*gold) {
                *gold -= resource.cost();
                *resources.write().unwrap().entry(resource).or_default() += 1;
            }

            drop(gold);
            thread::sleep(Duration::from_secs(1));
        });
    }

    // Create N consumers.
    for _ in 0..NCONSUMERS {
        let gold = Arc::clone(&gold);
        thread::spawn(move || loop {
            let spending = rand::thread_rng().gen_range(1..=3);
            let mut gold = gold.write().unwrap();

            if spending <= *gold {
                *gold -= spending;
            }

            drop(gold);
            thread::sleep(Duration::from_secs(1));
        });
    }

    for i in 0.. {
        let gold = *gold.read().unwrap();
        {
            let resources = resources.read().unwrap();
            println!("[State in iter {i}]\ngold: {gold}\nresources: {resources:?}\n");
        }
        thread::sleep(Duration::from_secs(1));
    }
}
