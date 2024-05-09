use mergesort::merge;
use std::thread;

pub fn mergesort(v: &[i32]) -> Vec<i32> {
    if v.len() == 1 {
        return v.to_vec();
    }

    let mid = v.len() >> 1;
    thread::scope(|s| {
        let l = s.spawn(|| mergesort(&v[..mid]));
        let r = s.spawn(|| mergesort(&v[mid..]));
        merge(&l.join().unwrap(), &r.join().unwrap())
    })
}
