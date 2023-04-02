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
    n:u64,
    m:u64,
        }
    if n * n < m {
        println!("-1");
        return;
    }
    let mut ans = std::u64::MAX;
    for i in 1..=n {
        let x = (m + i - 1) / i;
        if x <= n {
            ans = ans.min(i * x);
        }
        if i > x {
            break;
        }
    }
    if ans == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
