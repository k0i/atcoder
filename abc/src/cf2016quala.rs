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
    s:Chars,
    mut k:usize
        }
    let alphabet = "abcdefghijklmnopqrstuvwxyz"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i))
        .collect::<HashMap<char, usize>>();
    for i in 0..s.len() {
        if i == s.len() - 1 {
            k %= 26;
            let c = (alphabet[&s[i]] + k) % 26;
            println!("{}", alphabet.iter().find(|(_, &v)| v == c).unwrap().0);
            return;
        }
        if s[i] == 'a' {
            print!("a");
            continue;
        }
        if k >= 26 - alphabet[&s[i]] {
            k -= 26 - alphabet[&s[i]];
            print!("a");
        } else {
            print!("{}", s[i]);
        }
    }
}
