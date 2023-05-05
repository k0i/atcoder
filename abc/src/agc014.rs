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
    m:usize,
    ab:[(U1,U1);m]
        }
    let mut p = vec![0; n];
    for (a, b) in ab {
        p[a] += 1;
        p[b] += 1;
    }

    let ans = p.iter().all(|&x| x % 2 == 0);

    if ans {
        println!("YES");
    } else {
        println!("NO");
    }
}
