#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BinaryHeap, HashMap, HashSet},
};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    a:[(i64,i64);n],
        }
    let mut b = vec![];
    for i in 0..n {
        b.push((a[i].0 + a[i].1, i));
    }
    let mut taka = 0;
    let mut aoki = 0;
    b.sort_by_key(|x| Reverse(x.0));
    for i in 0..n {
        if i % 2 == 0 {
            taka += a[b[i].1].0;
        } else {
            aoki += a[b[i].1].1;
        }
    }
    println!("{}", taka - aoki);
}
