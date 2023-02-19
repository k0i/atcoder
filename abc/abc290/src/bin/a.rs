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
    n:usize,
    m:usize,
    a:[usize;n],
    b:[usize;m],
        }
    let mut sum = 0;
    for i in 0..m {
        sum += a[b[i] - 1];
    }
    println!("{}", sum);
}
