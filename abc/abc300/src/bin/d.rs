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
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    n:usize,
        }

    let mut root = 1;
    while root * root <= n {
        root += 1;
    }
    let mut e_sieve = vec![true; root + 10];
    e_sieve[0] = false;
    e_sieve[1] = false;
    for i in 2..root + 10 {
        if e_sieve[i] {
            let mut j = i * 2;
            while j < root + 10 {
                e_sieve[j] = false;
                j += i;
            }
        }
    }

    let mut prm = VecDeque::new();
    for i in 2..root + 10 {
        if e_sieve[i] {
            prm.push_back(i);
        }
    }
    let mut ans = 0;
    while prm.len() > 0 {
        let a = prm.pop_front().unwrap();
        if a > n || a * a * a * a * a > n {
            break;
        }
        for i in 0..prm.len() - 1 {
            let b = prm[i];
            if a * a * b * b * b > n {
                break;
            }
            for j in i + 1..prm.len() {
                let c = prm[j];
                if a * a * b * c * c > n {
                    break;
                }
                if a * a * b * c * c <= n {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}

fn is(n: usize, a: usize, b: usize, c: usize) -> bool {
    let mut ss = a;
    ss *= a;
    if ss > n {
        return false;
    }
    ss *= b;
    if ss > n {
        return false;
    }
    ss *= c;
    if ss > n {
        return false;
    }
    ss *= c;
    if ss == n {
        return true;
    }
    false
}
