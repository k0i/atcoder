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
    s:[Chars;8]
    }
    let row = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
    let col = vec!['1', '2', '3', '4', '5', '6', '7', '8'];
    let mut target = (0, 0);
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                target = (j, i);
                break;
            }
        }
    }
    target.1 = 7 - target.1;
    println!("{}{}", row[target.0], col[target.1]);
}
