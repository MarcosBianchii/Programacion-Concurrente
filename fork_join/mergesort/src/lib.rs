pub fn merge<T: Ord + Clone>(l: &[T], r: &[T]) -> Vec<T> {
    let mut res = Vec::with_capacity(l.len() + r.len());
    let (mut i, mut j) = (0, 0);

    while i < l.len() && j < r.len() {
        if l[i] < r[j] {
            res.push(l[i].clone());
            i += 1;
        } else {
            res.push(r[j].clone());
            j += 1;
        }
    }

    if i < l.len() {
        res.clone_from_slice(&l[i..]);
    }

    if j < r.len() {
        res.clone_from_slice(&r[j..]);
    }

    res
}
