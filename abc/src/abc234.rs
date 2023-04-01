#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{BinaryHeap, HashMap, HashSet};
#[fastout]
pub fn main() {
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
