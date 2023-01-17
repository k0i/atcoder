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
        }
    let mut ans = vec![0i64; n + 1];
    let mut tr = vec![];
    for i in 0..n {
        input! {
        a:i64,
        b:i64
                }
        tr.push((a, 1i64));
        tr.push((a + b, -1));
    }
    tr.sort();
    let mut acc = 0;
    for i in 0..tr.len() - 1 {
        acc += tr[i].1;
        ans[acc as usize] += tr[i + 1].0 - tr[i].0;
    }
    for i in 1..n + 1 {
        print!("{} ", std::cmp::max(ans[i], 0));
    }
}
