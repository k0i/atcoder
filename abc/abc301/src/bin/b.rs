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
    a:[i64;n],
        }
    let mut ans = vec![];
    let mut i = 0;
    while i < n {
        ans.push(a[i]);
        if i + 1 < n && (a[i] - a[i + 1]).abs() != 1 {
            if a[i] < a[i + 1] {
                for v in a[i] + 1..a[i + 1] {
                    ans.push(v);
                }
            } else {
                for v in (a[i + 1] + 1..a[i]).rev() {
                    ans.push(v);
                }
            }
        }
        i += 1;
    }

    for i in ans {
        print!("{} ", i);
    }
}
