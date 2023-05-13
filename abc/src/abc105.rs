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
        mut n: i64,
    }
    if n == 0 {
        println!("0");
        return;
    }
    let mut ans = String::new();
    while n != 0 {
        if n % 2 == 0 {
            ans = format!("0{}", ans);
        } else {
            n -= 1;
            ans = format!("1{}", ans);
        }
        n /= -2;
    }
    println!("{}", ans);
}
