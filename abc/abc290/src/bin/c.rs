#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    a:[usize;n],
        }
    let mut ind = HashMap::new();
    for i in 0..n {
        if a[i] < k {
            ind.insert(a[i], i);
        }
    }
    let mut keys = ind.keys().collect::<Vec<&usize>>();
    keys.sort();
    if keys.len() == 0 {
        println!("0");
        return;
    }
    if keys[0] != &0 {
        println!("0");
        return;
    }
    let mut pri = 0;
    for i in 1..keys.len() {
        if keys[i] - pri > 1 {
            println!("{}", pri + 1);
            return;
        }
        pri = *keys[i];
    }
    println!("{}", pri + 1);
}
