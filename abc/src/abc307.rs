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
    s:[Chars;n],
    }

    let mut ans = "No";
    for i in 0..n {
        'outer: for j in 0..n {
            if i == j {
                continue;
            }
            let new_chars = s[i].iter().chain(s[j].iter()).collect::<Vec<_>>();
            for l in 0..new_chars.len() {
                if new_chars[l] != new_chars[new_chars.len() - l - 1] {
                    continue 'outer;
                }
            }
            ans = "Yes";
        }
    }
    println!("{}", ans);
}
