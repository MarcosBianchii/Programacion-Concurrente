use rand::Rng;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use std_semaphore::Semaphore;

const NCLIENTS: u64 = 10;

fn main() {
    let barber = Arc::new(Semaphore::new(1));
    let lobby = Arc::new(Semaphore::new(0));
    let finished_cut = Arc::new(Semaphore::new(0));

    for id in 0..NCLIENTS {
        let barber = barber.clone();
        let lobby = lobby.clone();
        let finished_cut = finished_cut.clone();
        thread::spawn(move || {
            let mut randomizer = rand::thread_rng();
            thread::sleep(Duration::from_secs(randomizer.gen_range(1..=6 * NCLIENTS)));

            lobby.release();
            println!("[Client {id}] Waiting in lobby");

            barber.acquire();
            println!("[Client {id}] Woke up the barber");

            finished_cut.acquire();
            println!("[Client {id}] Exiting the barbershop");
        });
    }

    thread::spawn(move || {
        for _ in 0..NCLIENTS {
            let start = Instant::now();
            lobby.acquire();

            let slept = start.elapsed().as_secs_f32();
            println!("[Barber] Woke up, slept for {slept} seconds");

            // Cuts hair.
            thread::sleep(Duration::from_secs(2));
            println!("[Barber] Finished cut");
            finished_cut.release();

            barber.release();
        }
    })
    .join()
    .unwrap();
}
