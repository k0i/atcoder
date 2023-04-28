#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
        n: u64,
    };
    if n <= 2 {
        println!("0");
        return;
    }
    let mut res = n - 1;
    let mut x = 2;
    while x * x + 2 * x <= n {
        if n % x == 0 {
            res += (n - x) / x;
        }
        x += 1;
    }
    println!("{}", res);
}

fn c() {
    ip! {
        n: usize,
        s: [Chars; n],
    }

    let mut cnt_inner = 0;
    let mut n_a = 0;
    let mut n_b = 0;
    let mut n_c = 0;
    for si in s {
        let has_a = *si.last().unwrap() == 'A';
        let has_b = *si.first().unwrap() == 'B';
        if has_a && has_b {
            n_c += 1;
        } else {
            if has_a {
                n_a += 1;
            }
            if has_b {
                n_b += 1;
            }
        }
        cnt_inner += si.windows(2).filter(|x| *x == ['A', 'B']).count();
    }

    let mut cnt_bt = 0;
    if n_c > 1 {
        cnt_bt += n_c - 1;
        n_c = 1;
    }
    if n_c == 1 && (n_a > 0 || n_b > 0) {
        cnt_bt += 1;
    }
    cnt_bt += n_a.min(n_b);

    let ans = cnt_inner + cnt_bt;

    println!("{}", ans);
}
