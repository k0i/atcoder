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
        s:Chars
        }

    let mut ans = "No";
    let mut a = 0;
    let mut b = 0;
    for i in s {
        if i == 'o' {
            a += 1;
        } else if i == 'x' {
            b += 1;
        }
    }

    if a > 0 && b == 0 {
        ans = "Yes";
    }
    println!("{}", ans);
}
