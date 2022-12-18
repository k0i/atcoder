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
    mut s:Chars
        }
    let mut col = false;
    for i in 0..n {
        let cur = s[i];
        if cur == '"' {
            if col {
                col = false;
            } else {
                col = true;
            }
            continue;
        }
        if cur == ',' && !col {
            s[i] = '.';
            continue;
        }
    }
    for i in s {
        print!("{}", i);
    }
}
