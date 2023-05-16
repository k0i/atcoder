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
        n: usize,
        mut a: [usize; n]
    }

    let mut result = 0;
    for i in 0..n - 1 {
        if a[i] % 2 == 1 && a[i + 1] > 0 {
            a[i] -= 1;
            a[i + 1] -= 1;
            result += 1;
        }

        result += a[i] / 2;
    }

    result += a[n - 1] / 2;

    println!("{}", result);
}
