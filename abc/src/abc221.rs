#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};

// a^n % m を計算する O(log n)
fn powmod(a: i64, n: i64, m: i64) -> i64 {
    let mut ret = 1;
    let mut a = a;
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            ret = ret * a % m
        }
        a = a * a % m;
        n /= 2;
    }
    ret
}

#[inline]
fn lsb(i: usize) -> usize {
    i & i.wrapping_neg()
}

struct Fenwick {
    table: Vec<i64>,
}
impl Fenwick {
    pub fn new(size: usize) -> Self {
        Self {
            table: vec![0; size],
        }
    }
    pub fn prefix_sum(&self, mut i: usize, m: i64) -> i64 {
        let mut res = 0;
        while i != 0 {
            res += self.table[i];
            res %= m;
            i -= lsb(i);
        }
        res
    }
    pub fn add(&mut self, mut i: usize, x: i64, m: i64) {
        i += 1;
        while i < self.table.len() {
            self.table[i] += x;
            self.table[i] %= m;
            i += lsb(i);
        }
    }
}

#[fastout]
pub fn main() {
    input! {
      n: usize,
      a: [i64; n]
    }

    let mut a = a.into_iter().enumerate().collect_vec();

    a.sort_by_key(|v| v.1);

    let m: i64 = 998244353;
    let inv2 = powmod(2, m - 2, m);

    let mut fw = Fenwick::new(n);
    let mut ans: i64 = 0;
    for (idx, _) in a.into_iter() {
        ans += (fw.prefix_sum(idx, m) * powmod(2, idx as i64, m)) % m;
        ans %= m;
        fw.add(idx, powmod(inv2, idx as i64 + 1, m), m);
    }
    println!("{}", ans);
}

fn d() {
    input! {
    n:usize,
        }
    let mut ans = vec![0i64; n + 1];
    let mut tr = vec![];
    for i in 0..n {
        input! {
        a:i64,
        b:i64
                }
        tr.push((a, 1i64));
        tr.push((a + b, -1));
    }
    tr.sort();
    let mut acc = 0;
    for i in 0..tr.len() - 1 {
        acc += tr[i].1;
        ans[acc as usize] += tr[i + 1].0 - tr[i].0;
    }
    for i in 1..n + 1 {
        print!("{} ", std::cmp::max(ans[i], 0));
    }
}
