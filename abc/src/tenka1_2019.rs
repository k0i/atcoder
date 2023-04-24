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
        s:Chars
    }
    let w = s.iter().filter(|&c| *c == '.').count();
    let mut wi = 0;
    let mut ans = w;
    for (i, &c) in s.iter().enumerate() {
        if c == '.' {
            wi += 1;
        }
        ans = std::cmp::min(ans, i + 1 + w - 2 * wi)
    }
    println!("{}", ans)
}
