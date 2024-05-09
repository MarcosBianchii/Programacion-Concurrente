use rand::Rng;
use std::{
    sync::Arc,
    thread,
    time::{Duration, Instant},
};
use std_semaphore::Semaphore;

fn main() {
    const NCLIENTS: u64 = 10;

    let barber = Arc::new(Semaphore::new(1));
    let lobby = Arc::new(Semaphore::new(0));
    let finished_cut = Arc::new(Semaphore::new(0));

    // Spawn Barber.
    let bbarber = barber.clone();
    let blobby = lobby.clone();
    let bfinished_cut = finished_cut.clone();
    let barbershop = thread::spawn(move || {
        for _ in 0..NCLIENTS {
            let start = Instant::now();
            blobby.acquire();

            let slept = start.elapsed().as_secs_f32();
            println!("[Barber] Woke up, slept for {slept}");

            // Cuts hair.
            thread::sleep(Duration::from_secs(2));
            println!("[Barber] Finished cut");
            bfinished_cut.release();

            bbarber.release();
        }
    });

    (0..NCLIENTS)
        .map(|id| {
            let cbarber = barber.clone();
            let clobby = lobby.clone();
            let cfinished_cut = finished_cut.clone();
            thread::spawn(move || {
                let mut randomizer = rand::thread_rng();
                thread::sleep(Duration::from_secs(randomizer.gen_range(1..=6 * NCLIENTS)));

                clobby.release();
                println!("[Client {id}] Waiting in lobby");

                cbarber.acquire();
                println!("[Client {id}] Woke up the barber");

                cfinished_cut.acquire();
                println!("[Client {id}] Exiting the barbershop");
            })
        })
        .collect::<Vec<_>>()
        .into_iter()
        .for_each(|client| client.join().unwrap());

    barbershop.join().unwrap();
}
