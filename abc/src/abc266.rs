use std::collections::HashMap;

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
    tab:[(usize,usize,usize);n]
        }
    let max = 100001;
    let mut dp = vec![vec![std::i64::MIN; 5]; max];
    let mut snuke: HashMap<usize, (usize, usize)> = HashMap::new();
    for i in tab {
        let e = snuke.entry(i.0).or_insert((0, 0));
        e.0 = i.1;
        e.1 = i.2;
    }
    dp[0][0] = 0;
    for i in 1..max {
        for j in 0..5 {
            chmax!(dp[i][j], dp[i - 1][j]);
            if j != 0 {
                chmax!(dp[i][j], dp[i - 1][j - 1] as i64);
            }
            if j != 4 {
                chmax!(dp[i][j], dp[i - 1][j + 1] as i64);
            }
        }
        if let Some((x, a)) = snuke.get(&i) {
            dp[i][*x] += *a as i64;
        }
    }
    println!("{}", dp.iter().last().unwrap().iter().max().unwrap());
}
