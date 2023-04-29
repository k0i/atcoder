#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
        n:usize,
        m:usize,
        k:usize,
        abm:[(U1,U1);m],
        cdk:[(U1,U1);k]
    }
    let mut uf = UnionFind::new(n);
    let mut fs = vec![1; n];
    for (a, b) in abm {
        uf.union(a, b);
        fs[a] += 1;
        fs[b] += 1;
    }
    let g = uf.into_labeling();
    let mut mi = vec![0; n];
    for (c, d) in cdk {
        if g[c] == g[d] {
            mi[c] += 1;
            mi[d] += 1;
        }
    }
    let mut gs = vec![0; n];
    for &p in &g {
        gs[p] += 1;
    }
    println!("{}", (0..n).map(|x| gs[g[x]] - fs[x] - mi[x]).join(" "));
}
