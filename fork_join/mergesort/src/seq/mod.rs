use mergesort::merge;

pub fn mergesort(v: &[i32]) -> Vec<i32> {
    if v.len() == 1 {
        return v.to_vec();
    }

    let mid = v.len() >> 1;
    let l = mergesort(&v[..mid]);
    let r = mergesort(&v[mid..]);
    merge(&l, &r)
}
