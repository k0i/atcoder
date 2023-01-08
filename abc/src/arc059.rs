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
    a:[i64;n]
        }
    let avg = a.iter().sum::<i64>() as f64 / n as f64;
    let avg = avg.round() as i64;
    let mut ans = 0;
    for i in a {
        ans += (i - avg).pow(2);
    }
    println!("{}", ans);
}
