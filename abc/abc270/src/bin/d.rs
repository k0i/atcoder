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
fn main() {
    input! {
        mut n:usize,k:usize,a:[usize;k]
    }
    let mut dp = vec![0; n + 1];
    for i in 0..=n {
        for &r in a.iter() {
            if i == r {
                dp[i] = r;
                break;
            }
            if i < r {
                break;
            }
            chmax!(dp[i], i - dp[i - r]);
        }
    }
    println!("{}", dp[n]);
}
