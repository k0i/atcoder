#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    v:[U1;n],
        }
    let mut c = vec![vec![0; 100_000]; 2];
    for i in 0..n {
        c[i % 2][v[i]] += 1;
    }
    let mut v = vec![vec![]; 2];
    for i in 0..2 {
        for j in 0..100_000 {
            v[i].push((c[i][j], j));
        }
        v[i].sort_by_key(|&(x, _)| Reverse(x));
    }
    let mut res = n;
    for i in 0..2 {
        for j in 0..2 {
            if v[0][i].1 != v[1][j].1 {
                res = res.min(n - v[0][i].0 - v[1][j].0);
            }
        }
    }
    println!("{}", res);
}
