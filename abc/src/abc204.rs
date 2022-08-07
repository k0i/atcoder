use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
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
