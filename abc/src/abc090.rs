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
        n: i64,
        k: i64,
    }

    let mut ans = 0;
    for b in k + 1..=n {
        ans += n / b * (b - k);
        ans += (n % b - k + 1).max(0);
    }

    if k == 0 {
        ans -= n;
    }

    println!("{}", ans);
}
