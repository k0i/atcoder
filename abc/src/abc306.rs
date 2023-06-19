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
    xy:[(usize,isize);n]
        }
    let mut dp = vec![vec![-10000000000000000; 2]; n + 1];
    dp[0][0] = 0;
    let mut ans = std::isize::MIN;
    for i in 0..n {
        let (x, y) = xy[i];
        match x {
            0 => {
                dp[i + 1][0] = std::cmp::max(dp[i][0], std::cmp::max(dp[i][0], dp[i][1]) + y);
            }
            1 => {
                dp[i + 1][1] = std::cmp::max(dp[i][1], dp[i][0] + y);
            }
            _ => unreachable!(),
        }
        dp[i + 1][1] = std::cmp::max(dp[i][1], dp[i + 1][1]);
        dp[i + 1][0] = std::cmp::max(dp[i][0], dp[i + 1][0]);
        ans = std::cmp::max(ans, dp[i + 1][0]);
        ans = std::cmp::max(ans, dp[i + 1][1]);
    }
    println!("{}", ans);
}
