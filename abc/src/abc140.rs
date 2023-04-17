#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: Bytes,
    }
    s.dedup();
    let mut res = n - s.len();
    if k <= (s.len() - 1) / 2 {
        res += k * 2;
    } else {
        res += s.len() - 1;
    }
    println!("{}", res);
}
