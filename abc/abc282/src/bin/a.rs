use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let BIG_ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut ans = String::new();
    for i in 0..n {
        ans.push(BIG_ALPHABET.chars().nth(i % 26).unwrap());
    }
    for i in ans.chars() {
        print!("{}", i);
    }
}
