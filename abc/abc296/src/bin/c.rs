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
    x:isize,
    a:[isize;n]
        }
    let mut hs = HashSet::new();
    for i in 0..n {
        hs.insert(a[i]);
    }
    if x > 0 {
        for i in 0..n {
            let wanted = a[i] - x;
            if hs.contains(&wanted) {
                println!("Yes");
                return;
            }
        }
        println!("No");
    } else {
        for i in 0..n {
            let wanted = a[i] + x;
            if hs.contains(&wanted) {
                println!("Yes");
                return;
            }
        }
        println!("No");
    }
}
