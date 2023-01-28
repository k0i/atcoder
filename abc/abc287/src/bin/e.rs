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
    n: usize,
    }

    let mut s = vec![];

    for i in 0..n {
        input! {
            sr: String
        }
        s.push((sr, i));
    }
    s.sort_unstable();

    let mut ans = vec![0; n];
    for i in 0..n - 1 {
        let a = s[i].0.chars().collect_vec();
        let b = s[i + 1].0.chars().collect_vec();

        let mut cnt = 0;
        for i in 0..a.len().min(b.len()) {
            if a[i] != b[i] {
                break;
            }
            cnt += 1;
        }
        ans[s[i].1] = ans[s[i].1].max(cnt);
        ans[s[i + 1].1] = ans[s[i + 1].1].max(cnt);
    }

    for i in 0..n {
        println!("{}", ans[i]);
    }
}
