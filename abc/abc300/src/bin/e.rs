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
const MOD: i64 = 998244353;

#[fastout]
pub fn main() {
    ip! {
    n:usize
        }
    let mut dp = vec![vec![0; n + 1]; n + 1]; // dpテーブルを初期化
    dp[0][1] = 1; // 初期条件

    for i in 0..n {
        for j in 1..=n {
            for k in 1..=6 {
                let nj = j * k;
                if nj <= n {
                    dp[i + 1][nj] += dp[i][j];
                    dp[i + 1][nj] %= MOD;
                }
            }
        }
    }

    let ans = dp[n].iter().sum::<i64>() % MOD; // N回操作した後、持っている整数がNになる確率を求める
    println!("{}", ans);
}
