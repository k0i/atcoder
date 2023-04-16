#![allow(unused_imports)]
use itertools::Itertools;
use num::{BigInt, ToPrimitive, Zero};
use num_bigint::Sign;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet, VecDeque};
const MOD: usize = 998244353;

#[fastout]
pub fn main() {
    let mut s = VecDeque::new();
    let mut pow_10 = vec![1];
    for i in 1..600001 {
        pow_10.push(pow_10[i - 1] * 10 % MOD);
    }
    s.push_back(1);
    let mut m = 1;

    ip! {
        q: usize,
    }

    for _ in 0..q {
        ip! { c: usize }

        match c {
            1 => {
                ip! {x: usize}
                s.push_back(x);
                m = m * 10 + x;
                m %= MOD;
            }
            2 => {
                let x = s.pop_front().unwrap();
                m += MOD;
                m -= x * pow_10[s.len()] % MOD;
                m %= MOD;
            }
            _ => {
                println!("{}", m % MOD);
            }
        }
    }
}

fn pow(x: usize, n: usize) -> usize {
    let mut x = x;
    let mut n = n;
    let mut ans = 1;
    while n > 0 {
        if n & 1 == 1 {
            ans *= x;
            ans %= MOD;
        }
        x *= x;
        n >>= 1;
    }
    ans
}
