use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
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

pub fn main() {
    c()
}

fn d() {
    input! {n:usize,m:usize,x: [usize;n],d: [(usize,usize);m]}
    let mut dp = vec![vec![0; n + 1]; n + 1];
    let mut bonus = vec![0; n + 1];
    for (c, y) in d {
        bonus[c] += y;
    }

    for i in 1..n + 1 {
        for j in 1..=i {
            chmax!(dp[i][0], dp[i - 1][j]);
            dp[i][j] = dp[i - 1][j - 1] + x[i - 1] + bonus[j];
        }
    }
    let mut max = 0;
    for i in 1..=n {
        chmax!(max, dp[n][i]);
    }
    println!("{}", max);
}

fn c() {
    input! {
    n:usize,
    s:[String;n]
    }
    let mut h = HashMap::new();
    for i in s {
        let cnt = h.entry(i.clone()).or_insert(0);
        if cnt == &0 {
            println!("{}", i);
        } else {
            println!("{}({})", i, cnt);
        }
        *cnt += 1;
    }
}
