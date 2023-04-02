#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    a:[usize;n]
        }

    let mut g = vec![0; n + 1];

    for i in 0..n {
        g[a[i] - 1] += 1;
    }

    let mut q = VecDeque::new();
    for i in 0..n {
        if g[i] == 0 {
            q.push_back(i);
        }
    }

    let mut ans = n;
    while let Some(j) = q.pop_front() {
        ans -= 1;
        g[a[j] - 1] -= 1;
        if g[a[j] - 1] == 0 {
            q.push_back(a[j] - 1);
        }
    }

    println!("{}", ans);
}
