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
        s: Bytes
    }
    let mut arc = vec![];
    let le = l(&s);
    for w in le.windows(3) {
        if w[0].0 == b'A' && w[1].0 == b'R' && w[1].1 == 1 && w[2].0 == b'C' {
            arc.push((w[0].1).min(w[2].1));
        }
    }
    println!("{}", (arc.len() * 2).min(arc.iter().sum::<usize>()));
}
fn l(a: &[u8]) -> Vec<(u8, usize)> {
    if a.len() == 0 {
        return vec![];
    }
    let mut pre = a[0];
    let mut cnt = 1;
    let mut res = vec![];
    for i in 1..a.len() {
        if pre == a[i] {
            cnt += 1;
        } else {
            res.push((pre, cnt));
            pre = a[i];
            cnt = 1;
        }
    }
    res.push((pre, cnt));
    res
}
