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
        a:[i64;n],
        b:[i64;n]
    };
    let mut diff = 0;
    for i in 0..n {
        let d = b[i] - a[i];
        diff += if d > 0 { d >> 1 } else { d };
    }
    println!("{}", if diff >= 0 { "Yes" } else { "No" });
}
