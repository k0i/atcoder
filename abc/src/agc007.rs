#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    a:[Chars;h],
        }
    let mut cnt = 0;

    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                cnt += 1;
            }
        }
    }
    if cnt == h + w - 1 {
        println!("Possible");
    } else {
        println!("Impossible");
    }
}
