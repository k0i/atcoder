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
    x: usize,
    y: usize,
    ab: [(usize, usize); n]
    }
    let mut dp = vec![vec![std::usize::MAX; y + 1]; x + 1];
    dp[0][0] = 0;

    for (a, b) in ab {
        for i in (0..=x).rev() {
            for j in (0..=y).rev() {
                if dp[i][j] == std::usize::MAX {
                    continue;
                }
                let (nx, ny) = (std::cmp::min(x, i + a), std::cmp::min(y, j + b));
                dp[nx][ny] = std::cmp::min(dp[nx][ny], dp[i][j] + 1);
            }
        }
    }

    if dp[x][y] == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", dp[x][y]);
    }
}
