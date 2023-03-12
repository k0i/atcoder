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
        a: usize,
        x: usize,
        m: usize,
    }

    let ans = f(a, x, m);

    println!("{}", ans);
}

fn f(a: usize, x: usize, m: usize) -> usize {
    if x == 0 {
        return 0;
    }

    if x % 2 == 0 {
        return (f(a, x / 2, m) * (1 + powmod(a, x / 2, m))) % m;
    } else {
        return (1 + a * f(a, x - 1, m)) % m;
    }
}

fn powmod(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..64 {
        let w = 1 << i;
        if (b / w) % 2 == 1 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    ans
}
