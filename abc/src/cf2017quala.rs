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
    k:usize,
        }
    if k == 0 || k == n || k == m || k == n * m {
        println!("Yes");
        return;
    }
    for i in 0..m {
        for j in 0..n {
            if i * (n - j) + j * (m - i) == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
