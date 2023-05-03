#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    p:[usize;n],
        }
    let mut dp = vec![false; 10001];
    dp[0] = true;

    for i in 0..n {
        for j in (0..10001).rev() {
            if dp[j] {
                dp[j + p[i]] = true;
            }
        }
    }
    let ans = dp.iter().filter(|&x| *x).count();
    println!("{}", ans);
}
