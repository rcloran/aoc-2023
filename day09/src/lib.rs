pub use util::prelude::*;

pub fn succ_diff(l: &[isize], f: &impl Fn(&[isize], isize) -> isize) -> isize {
    if l.iter().all(|&v| v == 0) {
        return 0;
    }
    let mut diffs = vec![];
    for (a, b) in l.iter().tuple_windows() {
        diffs.push(b - a);
    }

    let below = succ_diff(&diffs, f);
    f(l, below)
}
