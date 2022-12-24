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
    }

    let mut a = 0_i64;
    let mut b = std::i64::MAX;

    for _ in 0..n {
        input! {l: i64, r: i64};
        a = a.max(l);
        b = b.min(r);

        let ans = if a <= b {
            0
        } else {
            {
                ((a - b) + 1) / 2
            }
        };

        println!("{}", ans);
    }
}
