#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::algo::{dijkstra, min_spanning_tree};
use petgraph::data::FromElements;
use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    x:usize,
    y:usize,
    z:usize,
    s:Chars
        }
    let mut dp = vec![vec![0; 2]; s.len() + 1];
    dp[0][1] = z;

    for i in 0..s.len() {
        let cur = s[i];
        match cur {
            'a' => {
                dp[i + 1][0] = std::cmp::min(dp[i][0] + x, dp[i][1] + z + x);
                dp[i + 1][1] = std::cmp::min(dp[i][1] + y, dp[i][0] + z + y);
            }
            'A' => {
                dp[i + 1][1] = std::cmp::min(dp[i][0] + z + x, dp[i][1] + x);
                dp[i + 1][0] = std::cmp::min(dp[i][1] + z + y, dp[i][0] + y);
            }
            _ => {}
        }
    }
    let ans = std::cmp::min(dp[s.len()][0], dp[s.len()][1]);
    println!("{}", ans);
}
