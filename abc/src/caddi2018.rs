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
    a:[usize;n],
        }
    let ans = if a.iter().all(|x| x % 2 == 0) {
        "second"
    } else {
        "first"
    };
    println!("{}", ans)
}
