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
            data.lock().unwrap().push("product");
            thread::sleep(Duration::from_secs(1));
        })
    };

    let consumer = thread::spawn(move || loop {
        let product = {
            let mut vec = data.lock().unwrap();
            vec.pop()
        };

        if let Some(product) = product {
            println!("got: {product}");
        }

        thread::sleep(Duration::from_secs(1));
    });

    for thread in [producer, consumer] {
        thread.join().unwrap();
    }
}
