use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n: usize,
    mut a: [i64; n]
    }
    let mut b = a.clone();
    (1..n).for_each(|i| a[i] += a[i - 1]);
    (0..n - 1).rev().for_each(|i| b[i] += b[i + 1]);
    let mut r = std::i64::MAX;
    (1..n).for_each(|i| r = r.min((a[i - 1] - b[i]).abs()));
    println!("{}", r);
}
