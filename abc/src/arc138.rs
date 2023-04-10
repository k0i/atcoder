#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    a:[usize;n],
        }
    let mut arr = vec![];
    let mut h_ind = HashMap::new();
    let mut max = 0;
    for i in k..n {
        if a[i] > max {
            arr.push(a[i]);
            h_ind.entry(a[i]).or_insert(i);
            max = a[i];
        }
    }
    arr.sort();

    let mut ans = std::usize::MAX;
    for i in 0..k {
        let ind = arr.upper_bound(&a[i]);
        if ind == arr.len() {
            continue;
        }
        let ind = h_ind[&arr[ind]];
        ans = ans.min(ind - i);
    }
    if ans == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
