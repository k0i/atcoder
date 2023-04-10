#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    a:[usize;n],
        }
    let mut h = HashMap::new();
    for i in 0..n {
        let v = h.entry(a[i]).or_insert(vec![]);
        v.push(i);
    }
    let mut ans = 0;
    for i in h.values() {
        if i.len() > 1 {
            ans += i.len() * (i.len() - 1) / 2;
        }
    }
    for k in 0..n {
        let v = h.get(&a[k]).unwrap();
        if v.len() > 1 {
            println!(
                "{}",
                ans - (v.len() * (v.len() - 1) / 2) + ((v.len() - 1) * (v.len() - 2) / 2)
            );
        } else {
            println!("{}", ans);
        }
    }
}
