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
    _n:usize,
    s:Chars
        }
    let mut start = (0, 0);
    let mut hss = HashSet::new();
    hss.insert(start);
    for i in s {
        match i {
            'L' => start.0 -= 1,
            'R' => start.0 += 1,
            'U' => start.1 += 1,
            _ => start.1 -= 1,
        }
        if hss.contains(&start) {
            println!("Yes");
            return;
        }
        hss.insert(start);
    }
    println!("No");
}
