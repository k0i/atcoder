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
    q:usize,
        }
    let mut ss = vec![0; 101];
    for _ in 0..q {
        ip! {
        a:usize,
        x:usize
        }
        match a {
            1 => ss[x] += 1,
            2 => ss[x] += 2,
            _ => {
                if ss[x] >= 2 {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
        }
    }
}
