use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    k:i64,
    t:usize,
    mut a:[i64;t],
        }
    a.sort_by(|a, b| b.cmp(a));
    let i = k - a[0];
    let mut ans = std::cmp::max(0, (a[0] - 1) - i);
    println!("{}", ans);
}
