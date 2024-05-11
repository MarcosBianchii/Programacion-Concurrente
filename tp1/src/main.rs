use std::{env, path::Path};

pub mod analyzing;

fn main() -> Result<(), String> {
    let nthreads = env::args().nth(1).ok_or("Use: cargo run -- [1..]")?;

    if let Ok(0) | Err(_) = nthreads.parse::<usize>() {
        Err("NTHREADS must be a number greater than 0")?;
    }

    // Tell rayon the amount of threads to use.
    env::set_var("RAYON_NUM_THREADS", nthreads);

    let path = Path::new("data");
    match analyzing::process_data(path) {
        Ok(stats) => println!("{stats}"),
        Err(e) => return Err(e.to_string()),
    }

    Ok(())
}
