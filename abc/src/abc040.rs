use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[allow(dead_code)]
fn update<T: PartialOrd>(value: T, target: &mut T, cond: std::cmp::Ordering) {
    if value.partial_cmp(target) == Some(cond) {
        *target = value;
    }
}
#[allow(unused_macros)]
macro_rules! chmax {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Greater)
    };
}
#[allow(unused_macros)]
macro_rules! chmin {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Less)
    };
}
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[i64;n],
        }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n - 1 {
        for j in 1..=2 {
            if i + j < n {
                chmin!(dp[i + j], dp[i] + (a[i + j] - a[i]).abs());
            }
        }
    }
    println!("{}", dp[n - 1]);
}
