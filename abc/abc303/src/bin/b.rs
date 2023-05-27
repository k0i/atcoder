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
    n:usize,
    m:usize,
    a:[[U1;n];m]
        }
    let mut hm = HashMap::new();
    for i in 0..m {
        for j in 1..n {
            let prev = a[i][j - 1];
            let cur = a[i][j];
            hm.entry(prev).or_insert(HashSet::new()).insert(cur);
            hm.entry(cur).or_insert(HashSet::new()).insert(prev);
        }
    }
    let mut comb = n * (n - 1) / 2;
    let mut ans = HashSet::new();
    for (k, v) in hm.iter() {
        for i in v.iter() {
            let a = k.min(i);
            let b = k.max(i);
            ans.insert((a, b));
        }
    }
}
