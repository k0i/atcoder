#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    ip! {
    n:usize,
     mut a:[[i64;n];n],
        b:[[i64;n];n]
        }

    let mut ans = "No";
    'outer: for i in 0..n {
        for j in 0..n {
            if a[i][j] != b[i][j] {
                break 'outer;
            }
        }
        if i == n - 1 {
            println!("Yes");
            return;
        }
    }

    'outer: for w in 0..10 {
        let mut aa = a.clone();
        for i in 1..=n {
            for j in 1..=n {
                a[i - 1][j - 1] = aa[n + 1 - j - 1][i - 1];
            }
        }
        for j in 0..n {
            for k in 0..n {
                if a[j][k] == 1 && b[j][k] != 1 {
                    continue 'outer;
                }
            }
        }
        ans = "Yes";
        break;
    }
    println!("{}", ans);
}
