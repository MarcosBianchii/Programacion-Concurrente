use rand::Rng;
use std::{
    sync::{Arc, Barrier, RwLock},
    thread,
    time::Duration,
};

const NFRIENDS: usize = 5;
const INITIAL_MONEY: f64 = 1000.0;

fn main() {
    let total = Arc::new(RwLock::new(INITIAL_MONEY));
    let distribute = Arc::new(Barrier::new(NFRIENDS));
    let week_start = Arc::new(Barrier::new(NFRIENDS));

    (0..NFRIENDS)
        .map(|i| {
            let total = total.clone();
            let distribute = distribute.clone();
            let week_start = week_start.clone();
            thread::spawn(move || {
                let mut week = 0;
                while *total.read().unwrap() > 1.0 {
                    distribute.wait();

                    let loan = *total.read().unwrap() / NFRIENDS as f64;
                    println!("{week}: [{i}] starts week with: {loan}");

                    week_start.wait();

                    if let Ok(mut total) = total.write() {
                        *total -= loan;
                    }

                    thread::sleep(Duration::from_secs(2));
                    let mut rng = rand::thread_rng();
                    let wins = loan * (rng.gen::<f64>() + 0.5);
                    println!("{week}: [{i}] winnings are: {wins}");

                    if let Ok(mut total) = total.write() {
                        *total += wins;
                    }

                    week += 1;
                }
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|thread| thread.join().unwrap())
}
