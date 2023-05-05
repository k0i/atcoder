#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
const MOD: usize = 1000000007;

#[fastout]
pub fn main() {
    ip! {
        n: usize,
        k: usize,
    }
    let mut dp = vec![vec![0; n + 1]; n + 1];
    for i in 0..=n {
        dp[i][0] = 1;
    }
    for i in 1..=n {
        for j in 1..=n {
            dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
            dp[i][j] %= MOD;
        }
    }

    for i in 1..=k {
        println!("{}", (dp[n - k + 1][i] * dp[k - 1][i - 1]) % MOD);
    }
}
