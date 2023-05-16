#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {
        mut h: usize, mut w: usize,
        a: [Bytes; h],
    }
    let mut counts = [0; 26];
    for row in a {
        row.iter().for_each(|u| counts[(u - b'a') as usize] += 1);
    }
    let mut available = || {
        for _ in 0..(h / 2) * (w / 2) {
            if let Some(c) = counts.iter_mut().find(|c| **c >= 4) {
                *c -= 4;
            } else {
                return false;
            }
        }
        for _ in 0..(h % 2) * w / 2 + (w % 2) * h / 2 {
            if let Some(c) = counts.iter_mut().find(|c| **c >= 2) {
                *c -= 2;
            } else {
                return false;
            }
        }
        true
    };
    println!("{}", if available() { "Yes" } else { "No" });
}

fn b() {
    input! {
    n:usize,
    m:usize,
    k:usize,
        }
    if k == 0 || k == n || k == m || k == n * m {
        println!("Yes");
        return;
    }
    for i in 0..m {
        for j in 0..n {
            if i * (n - j) + j * (m - i) == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
