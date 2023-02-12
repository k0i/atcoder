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
    a:[usize;n],
    m:usize,
    bb:[usize;m],
    x:usize,
        }
    let mut b = HashSet::new();
    let mut dp = vec![false; x + 1];
    dp[0] = true;
    for i in bb {
        b.insert(i);
    }
    for i in 0..n {
        let current = a[i];
        let mut j = 1;
        while current * j <= x {
            if b.contains(&(current * j)) {
                j += 1;
                continue;
            }
            if dp[current * (j - 1)] {
                dp[current * j] = true;
            }
            j += 1;
        }
    }

    for i in 0..=x {
        if dp[i] {
            for j in 0..n {
                if i + a[j] <= x {
                    if b.contains(&(i + a[j])) {
                        continue;
                    }
                    dp[i + a[j]] = true;
                }
            }
        }
    }
    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
