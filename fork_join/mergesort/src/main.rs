use std::{env, time::Instant};

pub mod par;
pub mod ray;
pub mod seq;

fn rand_range(n: usize) -> Vec<i32> {
    (0..n).map(|_| rand::random()).collect()
}

fn main() -> Result<(), &'static str> {
    let pow: u32 = env::args()
        .nth(1)
        .ok_or("Use: cargo run -- pow")?
        .parse()
        .map_err(|_| "N should be a positive number")?;

    let n = 10usize.pow(pow);
    let v = rand_range(n);
    println!("n: {n}");

    let start = Instant::now();
    seq::mergesort(&v);
    println!("seq: {:?}", Instant::elapsed(&start));

    if n < 5 {
        let start = Instant::now();
        par::mergesort(&v);
        println!("par: {:?}", Instant::elapsed(&start));
    }

    let start = Instant::now();
    ray::mergesort(&v);
    println!("ray: {:?}", Instant::elapsed(&start));

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::cmp::Reverse;

    use super::*;

    fn take_n(elements: impl Iterator<Item = i32>, n: usize) -> impl Iterator<Item = i32> {
        let mut acc = std::collections::BinaryHeap::with_capacity(n + 1);

        for e in elements {
            acc.push(Reverse(e));
            if acc.len() > n {
                acc.pop();
            }
        }

        acc.into_iter().map(|Reverse(e)| e)
    }

    #[test]
    fn take_k() {
        let v = rand_range(1000000);

        let start = Instant::now();
        let mut top10: Vec<_> = take_n(v.iter().copied(), 10).collect();
        top10.sort_unstable_by(|a, b| b.cmp(&a));
        println!(
            "10 greatest are: {top10:?}\ntook: {:?}",
            Instant::elapsed(&start)
        );

        let start = Instant::now();
        let mut top10 = Vec::from_iter(v);
        top10.sort_unstable_by(|a, b| b.cmp(&a));
        top10.resize(10, 0);
        println!(
            "10 greatest are: {top10:?}\ntook: {:?}",
            Instant::elapsed(&start)
        );
    }
}
