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
    d:[usize;n],
        }
    if d[0] != 0 {
        println!("0");
        return;
    }
    let max_dis = *d.iter().max().unwrap();
    let mut dist_count = vec![0i64; max_dis + 1];
    for i in 0..n {
        dist_count[d[i]] += 1;
    }

    if dist_count[0] != 1 {
        println!("0");
        return;
    }
    let mut ans = dist_count[0];
    for i in 1..=max_dis {
        let prev = dist_count[i - 1];
        let cur = dist_count[i];
        ans *= modpow(prev, cur as u32, 998244353);
        ans %= 998244353;
    }
    ans %= 998244353;
    println!("{}", ans);
}
#[allow(dead_code)]
fn modpow(mut a: i64, mut b: u32, m: i64) -> i64 {
    let mut ret = 1i64;
    while b > 0 {
        if b & 1 == 1 {
            ret = ret * a % m;
        }
        a = a * a % m;
        b >>= 1;
    }
    ret % m
}
#[allow(dead_code)]
fn moddiv(mut a: i64, mut b: i64, m: i64) -> i64 {
    a %= m;
    b = modpow(b, (m - 2) as u32, m);
    a * b % m
}
#[allow(dead_code)]
fn modncr(n: i64, r: i64, m: i64) -> i64 {
    let mut a = 1i64;
    let mut b = 1i64;
    for i in 1..=r {
        a = a * (n - i + 1) % m;
        b = b * i % m;
    }
    moddiv(a, b, m)
}
