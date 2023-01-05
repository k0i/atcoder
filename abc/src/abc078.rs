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
    m:usize
        }
    let mut ans = 1900 * m + 100 * (n - m);
    let mut x = 1;
    for _ in 0..m {
        x *= 2;
    }
    ans *= x;
    println!("{}", ans);
}
