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
        s: Chars,
    };
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    for c in s.iter() {
        match c {
            'R' => r += 1usize,
            'G' => g += 1usize,
            'B' => b += 1usize,
            _ => {}
        }
    }
    let mut ans = r * g * b;
    for j in 0..n {
        for i in 0..j {
            let k = 2 * j - i;
            if k >= n {
                continue;
            }
            if s[i] != s[j] && s[i] != s[k] && s[j] != s[k] && ans > 0 {
                ans -= 1;
            }
        }
    }
    println!("{}", ans);
}
