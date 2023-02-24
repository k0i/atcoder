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
        n: usize,
        k: usize,
        q: [(usize, usize); k],
    }
    let mut a: Vec<i32> = vec![0; n];
    for &(l, r) in &q {
        a[l - 1] += 1;
        if r < n {
            a[r] -= 1;
        }
    }
    for i in 0..n {
        if i > 0 {
            a[i] = a[i] + a[i - 1];
        }
        print!("{}", a[i] % 2);
    }
    println!();
}

fn a() {
    input! {
    w:usize,
    h:usize
    }
    let mut point = 0;
    if w % 4 == 0 {
        point = w / 4;
    }
    if h / point == 3 {
        println!("4:3");
    } else {
        println!("16:9");
    }
}
