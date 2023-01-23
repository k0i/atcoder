#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
fn check(n: usize) -> bool {
    let n_chars = n.to_string().chars().collect_vec();
    n_chars.contains(&'7') && n_chars.contains(&'5') && n_chars.contains(&'3')
}

fn rec(mut n: usize, up: usize, mut ans: usize) -> usize {
    if n > up {
        return ans;
    }
    if check(n) {
        ans += 1;
    }
    ans = rec(n * 10 + 3, up, ans);
    ans = rec(n * 10 + 5, up, ans);
    rec(n * 10 + 7, up, ans)
}
#[fastout]
pub fn main() {
    input! {
    n:usize,
        }
    let ans = rec(0, n, 0);
    println!("{}", ans);
}
