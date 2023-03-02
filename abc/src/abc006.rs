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
    n:usize,
        }
    println!("{}", tribonacci(n - 1) % 10007);
}

fn tribonacci(n: usize) -> usize {
    let mut t = vec![0, 0, 1];
    for i in 3..=n {
        t.push((t[i - 1] + t[i - 2] + t[i - 3]) % 10007);
    }
    t[n] % 10007
}
