#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    q:usize
        }
    let mut waited = BTreeSet::new();
    let mut already = BTreeSet::new();
    let mut next = 1;
    for _ in 0..q {
        ip! {
        a:usize
            }
        match a {
            1 => {
                waited.insert(next);
                next += 1;
            }
            2 => {
                ip! {
                x:usize,
                }
                already.insert(x);
                if waited.contains(&x) {
                    waited.remove(&x);
                }
            }
            _ => {
                let min = waited.iter().next().unwrap();
                println!("{}", min);
            }
        }
    }
}
