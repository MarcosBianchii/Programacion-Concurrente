use rand::Rng;
use std::{
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let ingredients = ["Paper", "Tobacco", "Matches"];
    let table = Arc::new(Mutex::new([false; 3]));
    let refill = Arc::new(Condvar::new());
    let call_agent = Arc::new(Condvar::new());

    for id in 0..3 {
        let table = table.clone();
        let refill = refill.clone();
        let call_agent = call_agent.clone();
        let name = ingredients[id];
        thread::spawn(move || loop {
            let mut table = refill
                .wait_while(table.lock().unwrap(), |table| {
                    !(0..3).all(|i| i == id || table[i])
                })
                .unwrap();

            *table = [false; 3];

            println!("[{name}] is smoking...\n");
            thread::sleep(Duration::from_secs(5));

            call_agent.notify_one();
        });
    }

    thread::spawn(move || loop {
        let mut table = call_agent
            .wait_while(table.lock().unwrap(), |table| table.iter().any(|&ing| ing))
            .unwrap();

        let ing = rand::thread_rng().gen_range(0..3);
        (0..3).for_each(|i| table[i] = i != ing);

        let ings: Vec<_> = (0..3)
            .filter_map(|i| (i != ing).then_some(ingredients[i]))
            .collect();

        println!("Refilled: {ings:?}");
        refill.notify_all();
    })
    .join()
    .unwrap();
}
