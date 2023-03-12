#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {s: Chars, k: usize}

    let n = s.len();
    let mut t = vec![];

    for i in 0..n {
        let mut v = String::new();
        for j in i..std::cmp::min(i + 10, n) {
            v.push(s[j]);

            t.push(v.clone())
        }
    }

    t.sort();
    t.dedup();

    println!("{}", t[k - 1])
}
