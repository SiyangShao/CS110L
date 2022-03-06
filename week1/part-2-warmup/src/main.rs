/* The following exercises were borrowed from Will Crichton's CS 242 Rust lab. */

use std::collections::{HashSet, HashMap};
// test2
fn main() {
    println!("Hi! Try running \"cargo test\" to run tests.");
}

fn add_n(v: Vec<i32>, n: i32) -> Vec<i32> {
    let mut v=v;
    for u in v.iter_mut() {
        *u += n;
    }
    v
}

fn add_n_inplace(v: &mut Vec<i32>, n: i32) {
    for u in v.iter_mut() {
        *u += n;
    }
}

fn dedup(v: &mut Vec<i32>) {
    // delete all but the first occurrence of each number
    let mut seen = HashSet::new();
    let mut i = 0;
    while i < v.len() {
        if seen.contains(&v[i]) {
            v.remove(i);
        } else {
            seen.insert(v[i]);
            i += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_n() {
        assert_eq!(add_n(vec![1], 2), vec![3]);
    }

    #[test]
    fn test_add_n_inplace() {
        let mut v = vec![1];
        add_n_inplace(&mut v, 2);
        assert_eq!(v, vec![3]);
    }

    #[test]
    fn test_dedup() {
        let mut v = vec![3, 1, 0, 1, 4, 4];
        dedup(&mut v);
        assert_eq!(v, vec![3, 1, 0, 4]);
    }
}
