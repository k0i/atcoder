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
    n:usize,
    m:usize,
    bc:[Chars;n],
        }
    let mut b = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            b[i][j] = bc[i][j].to_digit(10).unwrap();
        }
    }
    let mut ans = vec![vec![0; m]; n];
    for i in 0..n - 1 {
        for j in 1..m - 1 {
            if b[i][j] == 0 {
                continue;
            }
            ans[i + 1][j] = b[i][j];
            let temp = b[i][j];
            b[i][j] = 0;
            if i + 2 < n {
                b[i + 2][j] -= temp;
            }
            b[i + 1][j - 1] -= temp;
            b[i + 1][j + 1] -= temp;
        }
    }
    for i in 0..n {
        for j in 0..m {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
