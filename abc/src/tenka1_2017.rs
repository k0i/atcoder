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
        n: i64,
    }
    for i in 1..=3500 {
        for j in 1..=3500 {
            let num = n * i * j;
            let den = 4 * i * j - n * (i + j);
            if den > 0 && num % den == 0 {
                println!("{} {} {}", i, j, num / den);
                return;
            }
        }
    }
}
