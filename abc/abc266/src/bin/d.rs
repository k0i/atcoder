use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    d:[(usize,i64,i64);n]
        }
    let mut dp = vec![vec![0; n + 1]; n + 1];
}

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
