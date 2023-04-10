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
    mut  s:[Chars;h]
         }
    for i in 0..h {
        let mut j = 0;
        while j < w {
            if s[i][j] == 'T' && j + 1 < w && s[i][j + 1] == 'T' {
                s[i][j] = 'P';
                s[i][j + 1] = 'C';
                j += 2;
            } else {
                j += 1;
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
