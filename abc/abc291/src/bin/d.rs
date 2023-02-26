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
        ab:[(usize,usize);n]
        }
    let mut dp = vec![vec![0; 2]; n];
    let mut m = 998244353;
    dp[0][0] = 1;
    dp[0][1] = 1;
    for i in 1..n {
        let (prev_a, prev_b) = ab[i - 1];
        let (a, b) = ab[i];
        if a != prev_a {
            dp[i][0] += dp[i - 1][0];
            dp[i][0] %= m;
        }
        if a != prev_b {
            dp[i][0] += dp[i - 1][1];
            dp[i][0] %= m;
        }
        if b != prev_a {
            dp[i][1] += dp[i - 1][0];
            dp[i][1] %= m;
        }
        if b != prev_b {
            dp[i][1] += dp[i - 1][1];
            dp[i][1] %= m;
        }
    }
    let mut ans = dp[n - 1][0] + dp[n - 1][1];
    ans %= m;
    println!("{}", ans);
}
