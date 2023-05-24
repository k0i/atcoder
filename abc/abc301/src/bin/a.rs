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
    s:Chars
    }
    let mut takaki = 0;
    let mut aoki = 0;
    for i in 0..n {
        match s[i] {
            'T' => takaki += 1,
            'A' => aoki += 1,
            _ => (),
        }
    }
    if takaki == aoki {
        let ans = takaki;
        let mut takaki = 0;
        let mut aoki = 0;
        for i in s {
            match i {
                'T' => takaki += 1,
                'A' => aoki += 1,
                _ => (),
            }
            if takaki == ans {
                println!("T");
                return;
            }
            if aoki == ans {
                println!("A");
                return;
            }
        }
    } else if takaki > aoki {
        println!("T");
    } else {
        println!("A");
    }
}
