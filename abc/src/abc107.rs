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
        x: [i32; n],
    }
    let (mut mi, mut pl) = (vec![0], vec![0]);
    for x in x {
        if x < 0 {
            mi.push(-x);
        } else {
            pl.push(x);
        }
    }
    mi.sort();
    pl.sort();
    let mut ans = 1_000_000_000;

    for i in 0..(mi.len()).min(k) {
        let j = k - i;
        if j < pl.len() {
            ans = ans.min(mi[i] * 2 + pl[j]);
        }
    }

    for i in 0..(pl.len()).min(k) {
        let j = k - i;
        if j < mi.len() {
            ans = ans.min(pl[i] * 2 + mi[j]);
        }
    }
    println!("{}", ans);
}
