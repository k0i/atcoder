#![allow(unused_imports)]
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use proconio::{
fastout, input,
marker::{Bytes, Chars, Isize1, Usize1},
};
fn dfs(v: usize, p: usize, g: &[<Vec<usize>], res: &mut [i64]) {
    for &u in &g[v] {
        if u != p {
            res[u] += res[v];
            dfs(u, v, g, res);
        }
    }
}
 
#[fastout]
pub fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut g = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
        g[b].push(a);
    }
    let mut res = vec![0; n];
    for _ in 0..q {
        input! {
            p: Usize1,
            x: i64,
        }
        res[p] += x;
    }
    dfs(0, n, &g, &mut res);
    println!("{}", res.iter().map(|x| x.to_string()).join(" "));
}
