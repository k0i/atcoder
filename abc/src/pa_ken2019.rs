use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[allow(dead_code)]
fn update<T: PartialOrd>(value: T, target: &mut T, cond: std::cmp::Ordering) {
    if value.partial_cmp(target) == Some(cond) {
        *target = value;
    }
}
#[allow(unused_macros)]
macro_rules! chmax {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Greater)
    };
}
#[allow(unused_macros)]
macro_rules! chmin {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Less)
    };
}

#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
        s:[Chars;5]
        }
    let mut dp = vec![vec![std::i64::MAX; 3]; n + 1];

    for i in 0..n {
        let mut rbw = vec![0; 3];
        for j in 0..5 {
            if s[j][i] == 'R' {
                rbw[0] += 1;
            } else if s[j][i] == 'B' {
                rbw[1] += 1;
            } else if s[j][i] == 'W' {
                rbw[2] += 1;
            }
        }
        if i == 0 {
            dp[0][0] = 5 - rbw[0];
            dp[0][1] = 5 - rbw[1];
            dp[0][2] = 5 - rbw[2];
            continue;
        }
        for j in 0..=2 {
            for k in 0..=2 {
                if j != k {
                    chmin!(dp[i][k], dp[i - 1][j] + 5 - rbw[k])
                }
            }
        }
    }
    println!("{}", dp[n - 1].iter().min().unwrap());
}
