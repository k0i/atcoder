#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        uv: [(Usize1, Usize1); m],
    }
    let mut g = vec![vec![]; n];
    let mut c = vec![0; n];
    for (u, v) in uv {
        g[u].push(v);
        g[v].push(u);
        c[u] += a[v];
        c[v] += a[u];
    }
    let mut que = BinaryHeap::new();
    for i in 0..n {
        que.push((Reverse(c[i]), i));
    }
    let mut erased = vec![false; n];
    let mut ans = 0;
    while let Some((Reverse(cost), u)) = que.pop() {
        if erased[u] {
            continue;
        }
        erased[u] = true;
        ans = std::cmp::max(ans, cost);
        for &v in &g[u] {
            c[v] -= a[u];
            que.push((Reverse(c[v]), v));
        }
    }
    println!("{}", ans);
}
