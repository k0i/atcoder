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
      b: i64,
      c: i64
    };
    let mut p = vec![];
    p.push((b - c / 2, b));
    p.push((-b - (c - 1) / 2, -b));
    p.push((-b, -b + (c - 1) / 2));
    if c > 1 {
        p.push((b, b + (c - 2) / 2));
    }
    p.sort();
    p.dedup_by(|a, b| {
        a.0 <= b.1 && {
            b.1 = b.1.max(a.1);
            true
        }
    });
    let ans = p.iter().map(|p| p.1 - p.0 + 1).sum::<i64>();
    println!("{}", ans);
}
