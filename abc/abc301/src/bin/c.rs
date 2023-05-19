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
    t:Chars,
        }
    let mut s_cnt = HashMap::new();
    let mut t_cnt = HashMap::new();
    let mut changeable = HashSet::new();
    for i in vec!['a', 't', 'c', 'o', 'd', 'e', 'r'] {
        changeable.insert(i);
    }
    let mut all = HashSet::new();

    for i in s.clone() {
        *s_cnt.entry(i).or_insert(0) += 1;
        all.insert(i);
    }
    for i in t.clone() {
        *t_cnt.entry(i).or_insert(0) += 1;
        all.insert(i);
    }
    for k in all {
        if k == '@' {
            continue;
        }
        let sv = s_cnt.get(&k).unwrap_or(&0);
        let tv = t_cnt.get(&k).unwrap_or(&0);
        if tv == sv {
            continue;
        }
        if !changeable.contains(&k) {
            println!("No");
            return;
        }
        match sv > tv {
            true => {
                if !t_cnt.contains_key(&'@') {
                    println!("No");
                    return;
                }
                let at_value = t_cnt.get(&'@').unwrap();
                if at_value >= &(sv - tv) {
                    t_cnt.insert('@', at_value - (sv - tv));
                    continue;
                } else {
                    println!("No");
                    return;
                }
            }
            false => {
                if !s_cnt.contains_key(&'@') {
                    println!("No");
                    return;
                }
                let at_value = s_cnt.get(&'@').unwrap();
                if at_value >= &(tv - sv) {
                    s_cnt.insert('@', at_value - (tv - sv));
                    continue;
                } else {
                    println!("No");
                    return;
                }
            }
        }
    }

    println!("Yes");
}
