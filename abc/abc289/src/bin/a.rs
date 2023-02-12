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
    mut s:Chars
        }

    for i in 0..s.len() {
        if s[i] == '1' {
            s[i] = '0';
        } else if s[i] == '0' {
            s[i] = '1';
        }
    }
    println!("{}", s.iter().collect::<String>());
}
