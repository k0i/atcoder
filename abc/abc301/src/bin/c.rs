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
    s:Chars,
    t:Chars
        }
}
