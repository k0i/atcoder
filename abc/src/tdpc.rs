#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    b()
}

fn b() {
    ip! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut dp = vec![vec![0_i64; m + 1]; n + 1];
    dp[0][0] = 0;
    let INF = 1000000000;
    for i in 0..n + 1 {
        for j in 0..m + 1 {
            if i == 0 && j == 0 {
                continue;
            }
            if (n + m - i - j) % 2 == 0 {
                dp[i][j] = -1 * INF;
                if i > 0 {
                    dp[i][j] = dp[i][j].max(dp[i - 1][j] + a[n - i] as i64);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].max(dp[i][j - 1] + b[m - j] as i64);
                }
            } else {
                dp[i][j] = INF;
                if i > 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j] - a[n - i] as i64);
                }
                if j > 0 {
                    dp[i][j] = dp[i][j].min(dp[i][j - 1] - b[m - j] as i64);
                }
            }
        }
    }
    let mut sum = 0_i64;
    for i in 0..n {
        sum += a[i] as i64;
    }
    for i in 0..m {
        sum += b[i] as i64;
    }
    println!("{}", (sum + dp[n][m]) / 2);
}

fn a() {
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
