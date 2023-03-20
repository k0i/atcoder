#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    h:usize,
    w:usize,
    a:[[usize;w];h],
        }
    let mut alphabet_upper = ".ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect::<Vec<char>>();

    let mut ans = vec![vec!['.'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                continue;
            }
            ans[i][j] = alphabet_upper[a[i][j]];
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
