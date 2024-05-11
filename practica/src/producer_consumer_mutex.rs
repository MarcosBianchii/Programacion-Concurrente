//! Implementation of the Producer / Consumer problem using a Mutex.

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let data = Arc::new(Mutex::new(vec![]));

    let producer = {
        let data = data.clone();
        thread::spawn(move || loop {
            let mut vec = data.lock().unwrap();
            vec.push("product");
            drop(vec);
            thread::sleep(Duration::from_secs(1));
        })
    };

    let consumer = {
        let data = data.clone();
        thread::spawn(move || loop {
            let mut vec = data.lock().unwrap();
            let product = vec.pop();
            drop(vec);

            println!("got: {product:?}");
            thread::sleep(Duration::from_secs(1));
        })
    };

    for thread in [producer, consumer] {
        thread.join().unwrap();
    }
}
