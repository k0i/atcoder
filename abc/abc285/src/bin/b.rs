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
    s:Chars
        }
    let mut i = 1;
    while i < n {
        let mut ans = 0;
        for j in 0..n {
            if i + j >= n {
                break;
            }
            if s[j] != s[j + i] {
                ans = j + 1;
                continue;
            } else {
                break;
            }
        }
        println!("{}", ans);
        i += 1;
    }
}
