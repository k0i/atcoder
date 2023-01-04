use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    proconio::input! {
        h: usize,
        w: usize,
        field: [proconio::marker::Chars; h]
    };

    let mut dp = vec![vec![1 << 30usize; w]; h];

    dp[0][0] = if field[0][0] == '#' { 1 } else { 0 };

    for j in 1..h {
        dp[0][j] = dp[0][j - 1] + nessecity(0, j - 1, 0, j, &field);
    }

    for i in 1..h {
        dp[i][0] = dp[i - 1][0] + nessecity(i - 1, 0, i, 0, &field);
        for j in 1..w {
            dp[i][j] = std::cmp::min(
                dp[i - 1][j] + nessecity(i - 1, j, i, j, &field),
                dp[i][j - 1] + nessecity(i, j - 1, i, j, &field),
            );
        }
    }

    println! {"{}", dp[h-1][w-1]};
}

fn nessecity(i1: usize, j1: usize, i2: usize, j2: usize, field: &[Vec<char>]) -> usize {
    if field[i2][j2] == '.' || field[i1][j1] == field[i2][j2] {
        0
    } else {
        1
    }
}
