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
    an:[i64;n],
        }
    let mut bn = vec![0; n];
    for i in (1..=n).rev() {
        let mut acc = 0;
        for x in (i * 2..=n).step_by(i) {
            acc ^= bn[x - 1]
        }
        bn[i - 1] = acc ^ an[i - 1];
    }

    let m = (1..=n).filter(|&i| bn[i - 1] == 1).collect::<Vec<usize>>();
    println!("{}", m.len());
    if m.len() > 0 {
        println!(
            "{}",
            m.iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
