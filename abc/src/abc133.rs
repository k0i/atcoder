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
        n: usize,
        a: [isize; n],
    };

    let mut s = vec![0; n];

    for i in 0..n {
        if i % 2 == 0 {
            s[0] += a[i];
        } else {
            s[0] -= a[i];
        }
    }

    for i in 1..n {
        s[i] = 2 * a[i - 1] - s[i - 1];
    }

    for i in 0..n {
        print!("{} ", s[i]);
    }
}
