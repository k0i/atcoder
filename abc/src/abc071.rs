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
        mut s1:Chars,
        mut s2:Chars
    }
    s1.dedup();
    s2.dedup();

    let mut ans = if s1[0] != s2[0] { 6usize } else { 3 };
    for i in 1..s1.len() {
        if s1[i - 1] != s2[i - 1] && s1[i] != s2[i] {
            ans *= 3;
        } else if s1[i] != s2[i] {
            ans *= 2;
        } else if s1[i - 1] == s2[i - 1] {
            ans *= 2;
        }
        ans %= 1000000007;
    }
    println!("{}", ans);
}
