#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};

const MOD: i64 = 998244353;

#[fastout]
pub fn main() {
    ip! {
        n: U1,
        a: U1,
        b: U1,
        p: i64,
        q: i64,
    }
    let mut dp = vec![vec![vec![0; 2]; n + 1]; n + 1];
    for i in 0..n {
        for f in 0..2 {
            dp[n][i][f] = 1;
            dp[i][n][f] = 0;
        }
    }

    for i in (0..n).rev() {
        for j in (0..n).rev() {
            for k in 1..=p {
                dp[i][j][0] += dp[n.min(i + k as usize)][j][1];
            }
            dp[i][j][0] *= powmod(p, (MOD - 2) as u32, MOD);
            dp[i][j][0] %= MOD;

            for k in 1..=q {
                dp[i][j][1] += dp[i][n.min(j + k as usize)][0];
            }
            dp[i][j][1] *= powmod(q, (MOD - 2) as u32, MOD);
            dp[i][j][1] %= MOD;
        }
    }

    println!("{}", dp[a][b][0]);
}
#[allow(dead_code)]
fn powmod(mut a: i64, mut b: u32, m: i64) -> i64 {
    let mut ret = 1i64;
    while b > 0 {
        if b & 1 == 1 {
            ret = ret * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    ret % m
}
