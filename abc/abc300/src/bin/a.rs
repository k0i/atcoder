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
    a:usize,
    b:usize,
    c:[usize;n],
        }
    let ans = a + b;
    let mut i = 0;
    for j in 0..n {
        if c[j] == ans {
            i = j + 1;
            break;
        }
    }
    println!("{}", i);
}
