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
    n:usize,
    a:[i64;n],
        }
    let mut aa = vec![];
    for i in a {
        if i % 2 == 0 {
            aa.push(i);
        }
    }
    print!("{}", aa.iter().map(|x| x.to_string()).join(" "));
}
