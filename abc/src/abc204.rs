use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    d()
}
fn d() {
    input! {
    n:usize,
    t:[usize;n]
    }
    let sum = t.iter().sum::<usize>();
    let mut dp = vec![vec![false; sum + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let tt = t[i];
        for j in 0..sum + 1 {
            if dp[i][j] {
                dp[i + 1][j] = true;
                dp[i + 1][j + tt] = true;
            }
        }
    }
    let mut ans = 1000000000;
    for i in 0..sum + 1 {
        if dp[n][i] {
            ans = std::cmp::min(ans, std::cmp::max(i, sum - i));
        }
    }
    println!("{}", ans);
}

fn c() {
    input! {
    n:usize,m:usize,
    c:[(usize,usize);m]
        }
    let mut con = vec![vec![]; n];
    for (a, b) in c {
        con[a - 1].push(b - 1);
    }
    let mut res = 0;
    for i in 0..n {
        let mut v = vec![false; n];
        res += dfs(i, &con, &mut v);
    }
    println!("{}", res);
}

fn dfs(i: usize, c: &[Vec<usize>], visited: &mut [bool]) -> usize {
    let mut ans = 1;
    visited[i] = true;
    for j in c[i].clone() {
        if !visited[j] {
            ans += dfs(j, c, visited);
        }
    }
    ans
}
