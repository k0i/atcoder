#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let mut cnt = vec![vec![0; 10]; 10];
    for i in 1..=n {
        let i: i64 = i as i64;
        let btm = i % 10;
        let mut top = -1;
        let mut x = i;
        while 0 < x {
            top = x % 10;
            x /= 10;
        }

        cnt[top as usize][btm as usize] += 1;
    }

    let mut ans = 0;
    for i in 0..10 {
        for j in 0..10 {
            ans += cnt[i][j] * cnt[j][i]
        }
    }
    println!("{}", ans);
}
