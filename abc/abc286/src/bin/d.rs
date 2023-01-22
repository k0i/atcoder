#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
pub fn main() {
    input! {
     n:usize,
     x:usize,
    mut ab:[(usize,usize);n]
         }
    let mut dp = vec![false; x + 1];

    dp[0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        let mut tmp = dp.clone();
        for i in 1..=b {
            let cur = a * i;
            if cur > x {
                break;
            }
            for j in 0..=x {
                if cur + j > x {
                    break;
                }
                if dp[j] {
                    tmp[j + cur] = true;
                }
            }
        }
        dp = tmp;
    }
    if dp[x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
