#![allow(unused_imports)]
use itertools::Itertools;
use num::Integer;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};

#[fastout]
pub fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    };
    let mut m = a[0];
    let mut g = a[0];
    for i in 1..n {
        g = g.gcd(&a[i]);
        m = m.max(a[i]);
    }
    if k <= m && k % g == 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
