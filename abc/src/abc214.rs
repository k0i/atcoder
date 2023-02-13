#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};
fn solve() -> bool {
    input! {
        n: usize,
        mut lr: [(Usize1, Usize1); n],
    };
    lr.sort();
    let inf = 1_000_000_001;
    lr.push((inf, inf));
    let mut p = 0;
    let mut pq = BinaryHeap::new();
    for (l_i, r_i) in lr {
        while let Some(Reverse(r_j)) = pq.pop() {
            if p >= l_i {
                pq.push(Reverse(r_j));
                break;
            }
            if r_j < p {
                return false;
            }
            p += 1;
        }
        p = l_i;
        pq.push(Reverse(r_i));
    }
    true
}

#[fastout]
pub fn main() {
    input! {
        t: usize,
    };
    for _ in 0..t {
        println!("{}", if solve() { "Yes" } else { "No" });
    }
}
