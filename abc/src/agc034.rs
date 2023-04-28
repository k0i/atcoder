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
        mut s: Chars
    }

    let mut ans = 0usize;
    let mut a_count = 0;
    let mut i = 0;
    while i < s.len() {
        if s[i] == 'A' {
            a_count += 1;
            i += 1;
        } else if i < s.len() - 1 && s[i] == 'B' && s[i + 1] == 'C' {
            ans += a_count;
            i += 2;
        } else {
            a_count = 0;
            i += 1;
        }
    }

    println!("{}", ans);
}
