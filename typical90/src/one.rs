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
    l:usize,
    k:usize,
    a:[usize;n],
        }

    let mut left = 0;
    let mut right = l;
    while right - left > 1 {
        let mid = (left + right) / 2;
        let mut count = 0;
        let mut prev = 0;
        for i in 0..n {
            if a[i] - prev >= mid && l - a[i] >= mid {
                count += 1;
                prev = a[i];
            }
        }
        if count >= k {
            left = mid;
        } else {
            right = mid;
        }
    }
    println!("{}", left);
}
