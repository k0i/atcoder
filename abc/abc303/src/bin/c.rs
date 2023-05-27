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
    mut h:usize,
    k:usize,
    s:Chars,
    xy:[(isize,isize);m],
        }
    let mut start = (0, 0);
    let mut xyh = HashSet::new();
    for i in 0..m {
        xyh.insert(xy[i]);
    }

    for i in s {
        match i {
            'R' => {
                start.0 += 1;
            }
            'L' => {
                start.0 -= 1;
            }
            'U' => {
                start.1 += 1;
            }
            'D' => {
                start.1 -= 1;
            }
            _ => {}
        }
        if h == 0 {
            println!("No");
            return;
        }
        h -= 1;
        if xyh.contains(&start) && h < k {
            xyh.remove(&start);
            h = k;
        }
    }
    println!("Yes");
}
