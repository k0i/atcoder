#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    mut x:[usize;5*n]
        }
    x.sort();
    let mut a = VecDeque::new();
    for i in x {
        a.push_back(i);
    }
    for i in 0..n {
        a.pop_front();
    }
    for i in 0..n {
        a.pop_back();
    }
    let mut sum = 0;
    for i in a {
        sum += i;
    }
    let ans = sum as f64 / (3. * n as f64);
    println!("{}", ans);
}
