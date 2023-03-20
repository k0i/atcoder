#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    m:usize,
    a:[i64;n],
    b:[i64;m],
        }
    let mut min_heap = std::collections::BinaryHeap::new();
    let mut ahm = HashMap::new();
    let mut bhm = HashMap::new();
    for i in 0..n {
        min_heap.push(std::cmp::Reverse(a[i]));
        ahm.insert(a[i], i);
    }
    for i in 0..m {
        min_heap.push(std::cmp::Reverse(b[i]));
        bhm.insert(b[i], i);
    }
    let mut aans = vec![0; n];
    let mut bans = vec![0; m];
    let mut i = 1;
    while let Some(std::cmp::Reverse(x)) = min_heap.pop() {
        if ahm.contains_key(&x) {
            aans[ahm[&x]] = i;
        }
        if bhm.contains_key(&x) {
            bans[bhm[&x]] = i;
        }
        i += 1;
    }
    print!("{}", aans.iter().map(|x| x.to_string()).join(" "));
    println!();
    print!("{}", bans.iter().map(|x| x.to_string()).join(" "));
}
