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
use std::collections::{BTreeMap, HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    uv:[(U1,U1);n-1]
        }
    let mut g = vec![vec![]; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut ans = vec![];
    let mut lish = n;

    for i in 0..n {
        if g[i].len() > 2 {
            ans.push(g[i].len());
            lish -= g[i].len() + 1;
        }
    }
    for _ in 0..lish / 3 {
        ans.push(2);
    }
    ans.sort();

    println!("{}", ans.iter().join(" "));
}
