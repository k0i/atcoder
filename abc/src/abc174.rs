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
        }
    let mut a = vec![];
    a.push(7 % k);
    for i in 1..=k {
        a.push((a[i - 1] * 10 + 7) % k);
    }
    for i in 0..=k {
        if a[i] == 0 {
            println!("{}", i + 1);
            return;
        }
    }
    println!("-1");
}
