use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    k:usize,
    s:usize
        }

    let mut ans = 0;
    for i in 0..=k {
        for j in 0..=k {
            if i + j <= s && s - i - j <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
