use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};

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

#[fastout]
pub fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
    }
    let m = 1000_000_007;
    let x = modpow(2, n as u32, m) - 1;
    let y = modncr(n, a, m);
    let z = modncr(n, b, m);
    let answer = ((x - y + m) % m - z + m) % m;
    println!("{}", answer);
}
