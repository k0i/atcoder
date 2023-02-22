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
        mut p: [Usize1; n],
    }
    let mut q = vec![0; n];
    for i in 0..n {
        q[p[i]] = i;
    }
    let mut used = vec![false; n];
    let mut res = vec![];
    for i in 0..n {
        while p[i] != i {
            let a = i;
            let x = q[a];
            let y = x - 1;
            let b = p[y];
            if used[x] {
                println!("-1");
                return;
            }
            used[x] = true;
            res.push(x);
            p[x] = b;
            p[y] = a;
            q[a] = y;
            q[b] = x;
        }
    }
    if res.len() != n - 1 {
        println!("-1");
    } else {
        println!("{}", res.iter().map(|x| x.to_string()).join("\n"));
    }
}
