#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};

#[fastout]
pub fn main() {
    input! {t:i32}
    for _ in 0..t {
        input! {
            n:usize,
            m:usize,
            c:[usize;n],
            uv:[(Usize1,Usize1);m]
        }
        if c[0] == c[n - 1] {
            println!("-1");
            continue;
        }
        let mut g = vec![vec![vec![]; n]; n];
        for &(u, v) in &uv {
            for &(a, b) in &uv {
                if c[u] != c[a] && c[v] != c[b] {
                    g[u][a].push((v, b));
                    g[v][b].push((u, a));
                }
                if c[u] != c[b] && c[v] != c[a] {
                    g[u][b].push((v, a));
                    g[v][a].push((u, b));
                }
            }
        }
        let mut q = VecDeque::new();
        let mut d = vec![vec![-1; n]; n];
        q.push_back((0, n - 1));
        d[0][n - 1] = 0;
        while let Some((u, v)) = q.pop_front() {
            if (u, v) == (n - 1, 0) {
                break;
            }
            for &(a, b) in &g[u][v] {
                if d[a][b] >= 0 {
                    continue;
                }
                q.push_back((a, b));
                d[a][b] = d[u][v] + 1;
            }
        }
        println!("{}", d[n - 1][0])
    }
}
