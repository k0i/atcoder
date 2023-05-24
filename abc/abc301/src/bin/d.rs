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
use std::collections::{BTreeSet, HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    s:Chars,
    n:usize
    }
    let mut ans = 0;
    for (i, &ss) in s.iter().rev().enumerate() {
        if ss == '1' {
            ans |= 1 << i;
        }
    }
    if ans > n {
        println!("-1");
        return;
    }

    for (i, &ss) in s.iter().rev().enumerate().rev() {
        if ss == '?' {
            if ans | 1 << i <= n {
                ans |= 1 << i;
            }
        }
    }
    println!("{}", ans);
}
