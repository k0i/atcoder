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
    n:u64
        }
    let mut memo = vec![-1i64; 100010];
    println!("{}", solve(n, &mut memo));
}

fn solve(a: u64, memo: &mut [i64]) -> i64 {
    if a == 0 {
        return 0;
    }
    if memo[a as usize] != -1 {
        return memo[a as usize];
    }
    let mut res = std::i64::MAX;
    // x = 1
    chmin!(res, solve(a - 1, memo) + 1);
    //
    // x = 6
    let mut x = 6;
    while x <= a {
        chmin!(res, solve(a - x, memo) + 1);
        x *= 6;
    }
    let mut y = 9;
    while y <= a {
        chmin!(res, solve(a - y, memo) + 1);
        y *= 9;
    }
    memo[a as usize] = res;
    res
}
