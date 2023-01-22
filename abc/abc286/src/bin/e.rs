#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
const INF: usize = 1001001001;
pub fn main() {
    input! {
        n: usize,
        a: [usize; n],
        s: [Chars; n],
        q: usize,
        p: [(Usize1, Usize1); q],
    }
    let mut d = vec![vec![INF; n]; n];
    let mut c = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'Y' {
                d[i][j] = 1;
                c[i][j] = a[j];
            }
        }
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if d[i][k] + d[k][j] < d[i][j] {
                    d[i][j] = d[i][k] + d[k][j];
                    c[i][j] = c[i][k] + c[k][j];
                } else if d[i][k] + d[k][j] == d[i][j] && c[i][j] < c[i][k] + c[k][j] {
                    c[i][j] = c[i][k] + c[k][j];
                }
            }
        }
    }
    println!("{:?}", d);
    for &(u, v) in p.iter() {
        if d[u][v] == INF {
            println!("Impossible");
        } else {
            println!("{} {}", d[u][v], a[u] + c[u][v]);
        }
    }
}
