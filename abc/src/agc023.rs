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
    a:[i64;n],
        }
    let mut cmsm = vec![0];
    for i in 0..n {
        cmsm.push(cmsm[i] + a[i]);
    }
    let mut h = HashMap::new();
    for i in cmsm {
        h.entry(i).and_modify(|e| *e += 1).or_insert(1);
    }
    let mut ans = 0 as i64;
    for i in h {
        ans += i.1 * (i.1 - 1) / 2;
    }
    println!("{}", ans);
}
