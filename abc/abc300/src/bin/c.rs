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
    h:usize,
    w:usize,
    c:[Chars;h],
        }
    let m = h.min(w);
    let mut ans = vec![0; m];

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                let mut k = 1;
                if is_cross(&c, k as isize, i as isize, j as isize) {
                    while k < m && is_cross(&c, (k + 1) as isize, i as isize, j as isize) {
                        k += 1;
                    }
                    ans[k - 1] += 1;
                }
            }
        }
    }
    for i in 0..m {
        if i == m - 1 {
            println!("{}", ans[i]);
            return;
        }
        print!("{} ", ans[i]);
    }
}

//C[a][b] is #.
// C[a+d][b+d],C[a+d][b−d],C[a−d][b+d], and C[a−d][b−d] are all #, for all integers  d such that 1≤d≤n,
// At least one of C[a+n+1][b+n+1],C[a+n+1][b−n−1],C[a−n−1][b+n+1], and C[a−n−1][b−n−1] is .
fn is_cross(c: &Vec<Vec<char>>, cross_size: isize, i: isize, j: isize) -> bool {
    let h = c.len() as isize;
    let w = c[0].len() as isize;
    let dd = vec![
        (i + cross_size, j + cross_size),
        (i + cross_size, j - cross_size),
        (i - cross_size, j + cross_size),
        (i - cross_size, j - cross_size),
    ];
    for (ii, jj) in dd {
        if ii >= h || jj >= w {
            return false;
        }
        if ii < 0 || jj < 0 {
            return false;
        }
        if c[ii as usize][jj as usize] != '#' {
            return false;
        }
    }
    true
}
