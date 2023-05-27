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
    s:Chars,
    t:Chars
        }
    let mut ans = "Yes";
    for i in 0..n {
        if s[i] == t[i] {
            continue;
        }
        if s[i] == '1' {
            if t[i] == 'l' {
                continue;
            }
            ans = "No";
            break;
        }
        if t[i] == '1' {
            if s[i] == 'l' {
                continue;
            }
            ans = "No";
            break;
        }

        if s[i] == '0' {
            if t[i] == 'o' {
                continue;
            }
            ans = "No";
            break;
        }

        if t[i] == '0' {
            if s[i] == 'o' {
                continue;
            }
            ans = "No";
            break;
        }
        ans = "No";
        break;
    }
    println!("{}", ans);
}
