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
        s: Chars,
    }
    let mut cnt = 0;
    let mut zero = false;
    for &c in &s {
        if c == '+' {
            if !zero {
                cnt += 1;
            }
            zero = false;
        } else if c == '0' {
            zero = true;
        }
    }
    if !zero {
        cnt += 1;
    }
    println!("{}", cnt);
}
