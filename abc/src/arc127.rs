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
        n: i64,
    }
    let mut res = 0;
    let mut x = 1;
    for i in 0..15 {
        let mut xx = x;
        let mut yy = x + 1;
        for j in 0..=15 - i {
            res += (yy.min(n + 1) - xx).max(0);
            xx *= 10;
            yy *= 10;
        }
        x = x * 10 + 1;
    }
    println!("{}", res);
}
