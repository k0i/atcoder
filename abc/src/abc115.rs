#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut x: i64,
    }
    let mut sz = vec![1; n + 1];
    let mut pt = vec![1; n + 1];
    for i in 0..n {
        sz[i + 1] = sz[i] * 2 + 3;
        pt[i + 1] = pt[i] * 2 + 1;
    }
    println!("{}", f(n, x, &sz, &pt));
}
fn f(n: usize, x: i64, sz: &[i64], pt: &[i64]) -> i64 {
    if x <= 0 {
        0
    } else if n == 0 {
        if x > 0 {
            1
        } else {
            0
        }
    } else if sz[n] <= x {
        pt[n]
    } else {
        f(n - 1, x - 1, sz, pt)
            + f(0, x - 1 - sz[n - 1], sz, pt)
            + f(n - 1, x - 2 - sz[n - 1], sz, pt)
    }
}
