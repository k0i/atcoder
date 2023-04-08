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
        n: usize,
        a: [i64; n],
    }
    if n == 1 {
        println!("{}", a[0]);
        return;
    }

    let mut dp = vec![[0 as i64; 2]; n];
    const MOD: i64 = 1_000_000_007;

    dp[0][0] = (a[0] + a[1]) % MOD;
    dp[0][1] = (a[0] - a[1]) % MOD;
    let mut x = 1;
    let mut y = 1;

    for i in 0..n - 2 {
        x = (x + y) % MOD;
        y = (x - y) % MOD;
        dp[i + 1][0] = (dp[i][0] + dp[i][1] + x * a[i + 2]) % MOD;
        dp[i + 1][1] = (dp[i][0] - y * a[i + 2]) % MOD;
    }

    let res = ((dp[n - 2][0] + dp[n - 2][1]) % MOD + MOD) % MOD;
    println!("{}", res);
}
