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
pub fn main() {
    ip! {
    h:usize,
    w:usize,
    mut a:[Chars;h],
    b:[Chars;h],
        }
    for ii in 0..h {
        let mut aa = a.clone();
        for jj in 0..w {
            'A: for i in 0..h {
                for j in 0..w {
                    if aa[i][j] != b[i][j] {
                        break 'A;
                    }
                }
                if i == h - 1 {
                    println!("Yes");
                    return;
                }
            }
            let mut ca = vec![vec![]; h];
            for i in 0..h {
                ca[i] = aa[i].clone();
                ca[i].push(aa[i][0]);
                ca[i].remove(0);
            }
            aa = ca;
        }
        let mut ca = vec![vec![]; h];
        for i in 0..h - 1 {
            ca[i] = a[i + 1].clone();
        }
        ca[h - 1] = a[0].clone();
        a = ca;
    }
    println!("No");
}
