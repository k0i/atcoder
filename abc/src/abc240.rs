use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
                n:usize,
                a:[i64;n]
    }
    let mut stuck = vec![];
    let mut last = 0;
    for i in 0..n {
        if a[i] == *stuck.last().unwrap_or(&-1) {
            stuck.push(a[i]);
            last += 1;
            if last == a[i] {
                for i in 0..last {
                    stuck.pop();
                }
                if stuck.len() == 0 {
                    println!("{:?}", 0);
                    continue;
                }
                last = 0;
                let last_a = *stuck.last().unwrap();
                for j in (0..stuck.len()).rev() {
                    if stuck[j] == last_a {
                        last += 1;
                    } else {
                        break;
                    }
                }
            }
        } else {
            last = 1;
            stuck.push(a[i]);
        }
        println!("{:?}", stuck.len());
    }
}

fn c() {
    input! {
    n:usize,
    x:usize,
    ab:[(usize,usize);n],
            }
    let mut dp = vec![vec![0; x + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 0..x {
            if dp[i - 1][j] != 0 {
                if j + a < x + 1 {
                    dp[i][j + a] += dp[i - 1][j];
                }
                if j + b < x + 1 {
                    dp[i][j + b] += dp[i - 1][j];
                }
            }
        }
    }
    if dp[n][x] != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
