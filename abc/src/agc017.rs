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
    p:usize,
    a:[usize;n],
        }
    let mut sum = 0;
    let mut odd_cnt = 0;
    for i in 0..n {
        if a[i] % 2 == 1 {
            odd_cnt += 1;
        }
        sum += a[i];
    }
    let mut even_cnt = n - odd_cnt;
    if odd_cnt == 0 {
        if p == 0 {
            println!("{}", 2usize.pow(n as u32));
        } else {
            println!("0");
        }
        return;
    }

    println!("{}", 2usize.pow((n - 1) as u32));
}
