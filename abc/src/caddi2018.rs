#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    ip! {
      n:u64,
      mut p:u64,
    }
    let mut ans = 1;
    let mut l: u64 = 2;
    for i in 2..1000000 {
        let mut x: u64 = 0;
        while p % i == 0 {
            x += 1;
            p /= i;
        }
        ans *= i.pow((x / n) as u32);
        l += 1;
    }
    ans *= p.pow((1 / n) as u32);
    println!("{}", ans);
}

fn d() {
    ip! {
    n:usize,
    a:[usize;n],
        }
    let ans = if a.iter().all(|x| x % 2 == 0) {
        "second"
    } else {
        "first"
    };
    println!("{}", ans)
}
