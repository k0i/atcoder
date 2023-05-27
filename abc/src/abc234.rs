#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use superslice::Ext;
#[fastout]
pub fn main() {
    e()
}

fn e() {
    ip! {
    x:usize
    }
    let mut ss = BTreeSet::new();
    for first in 1..=9 {
        for d in -9..=8 {
            let mut s = String::new();
            let mut cur = first;
            for i in 0..18 {
                s.push_str(&cur.to_string());
                let int_s = s.parse::<usize>().unwrap();
                ss.insert(int_s);
                cur += d;
                if cur < 0 || cur > 9 {
                    break;
                }
            }
        }
    }
    let s = ss.into_iter().collect::<Vec<_>>();
    let ans = s.lower_bound(&x);
    println!("{}", s[ans]);
}

fn d() {
    ip! {
    n:usize,
    k:U1,
    p:[usize;n],
        }
    let mut min_hp = BinaryHeap::new();
    for i in 0..=k {
        min_hp.push(std::cmp::Reverse(p[i]));
    }

    println!("{}", min_hp.peek().unwrap().0);

    for i in k + 1..n {
        let min = min_hp.peek().unwrap().0;
        if min < p[i] {
            min_hp.pop();
            min_hp.push(std::cmp::Reverse(p[i]));
        }
        println!("{}", min_hp.peek().unwrap().0);
    }
}
