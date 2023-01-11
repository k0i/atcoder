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
    mut s:[Chars;n]
        }
    for i in 0..n {
        s[i].reverse();
    }
    s.sort();
    for i in 0..n {
        println!("{}", s[i].iter().rev().collect::<String>());
    }
}
