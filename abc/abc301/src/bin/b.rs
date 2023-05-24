#![allow(unused_imports)]
use either::Either::{Left, Right};
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
    while i + 1 < n {
        if (a[i] - a[i + 1]).abs() == 1 {
            ans.push(a[i]);
            ans.push(a[i + 1]);
            i += 2;
            continue;
        }
        let left = a[i];
        let right = a[i + 1];
        if left > right {
            let mut candidate = left;
            while candidate > right {
                ans.push(candidate);
                candidate -= 1;
            }
        } else {
            let mut candidate = left;
            while candidate < right {
                ans.push(candidate);
                candidate += 1;
            }
        }
        i += 1;
    }

    for i in ans {
        print!("{} ", i);
    }
}
