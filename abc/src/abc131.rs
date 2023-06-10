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
    mut ab:[(u64,u64);n]
        }
    ab.sort_by(|a, b| a.1.cmp(&b.1));
    let mut time = 0;
    for i in 0..n {
        time += ab[i].0;
        if time > ab[i].1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
