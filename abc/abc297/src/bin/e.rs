#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet},
};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    mut k:usize,
    a:[usize;n]
        }
    k += 1;
    let mut min = BinaryHeap::new();
    min.push(Reverse(0));
    let mut ans = vec![-1; k + 1];
    for i in 1..=k {
        let mut m = min.pop().unwrap().0;
        while ans[i - 1] == m {
            m = min.pop().unwrap().0;
        }
        ans[i] = m;
        for j in 0..n {
            min.push(Reverse(m + a[j] as i64));
        }
    }
    println!("{}", ans[k]);
}
