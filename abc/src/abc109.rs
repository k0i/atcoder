#![allow(unused_imports)]
use itertools::Itertools;
use num_integer::Integer;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};

#[fastout]
pub fn main() {
    input! {
        h : usize,
        w : usize,
        mut a : [[usize; w]; h]
    }

    let mut ans = Vec::with_capacity(h * w);
    for i in 0..h {
        let hsum = a[i].iter().sum::<usize>();

        if i + 1 < h && hsum.is_odd() {
            let first = a[i].iter().position(|&a| a.is_odd()).unwrap();
            a[i][first] -= 1;
            a[i + 1][first] += 1;
            ans.push((i, first, i + 1, first));
        }

        for j in 0..w {
            if j + 1 < w && a[i][j].is_odd() {
                a[i][j] -= 1;
                a[i][j + 1] += 1;
                ans.push((i, j, i, j + 1));
            }
        }
    }

    println!(
        "{}\n{}",
        ans.len(),
        ans.iter()
            .map(|&(i, j, k, l)| format!("{} {} {} {}", i + 1, j + 1, k + 1, l + 1))
            .join("\n")
    );
}
