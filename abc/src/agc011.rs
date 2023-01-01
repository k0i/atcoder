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
        mut a: [i64; n],
    }

    a.sort();

    let mut result = n;
    let mut sum = 0;

    for i in 0..n - 1 {
        sum += a[i];
        if sum * 2 < a[i + 1] {
            result = n - i - 1;
        }
    }
    println!("{}", result);
}
