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
    s:[String;n]
        }
    let mut ss = 0;
    for i in s {
        if i == "For" {
            ss += 1;
        }
    }
    if ss * 2 > n {
        println!("Yes");
    } else {
        println!("No");
    }
}
