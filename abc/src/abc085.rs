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
    mut h:usize,
    ab:[(usize,usize);n],
        }
    let mut a = vec![];
    let mut b = vec![];
    for i in 0..n {
        a.push(ab[i].0);
        b.push(ab[i].1);
    }
    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));
    if a[0] >= b[0] {
        println!(
            "{}",
            if h % a[0] == 0 {
                h / a[0]
            } else {
                h / a[0] + 1
            }
        );
        return;
    }
    let mut i = 0;
    let mut sum = 0;
    while b[i] > a[0] && i < n {
        sum += b[i];
        if h <= sum {
            println!("{}", i + 1);
            return;
        }
        i += 1;
        if i == n {
            break;
        }
    }
    let mut ans = i;
    h -= sum;
    ans += if h % a[0] == 0 {
        h / a[0]
    } else {
        h / a[0] + 1
    };
    println!("{}", ans);
}
