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
    s:Chars
        }
    let mut cur = s[0];
    for i in 1..n {
        if s[i] != cur {
            cur = s[i];
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
