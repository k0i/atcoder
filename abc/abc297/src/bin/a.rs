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
    d:i64,
    t:[i64;n],
        }
    let mut ans = -1;
    for i in 1..n {
        if t[i] - t[i - 1] <= d {
            ans = t[i];
            break;
        }
    }
    println!("{}", ans);
}
