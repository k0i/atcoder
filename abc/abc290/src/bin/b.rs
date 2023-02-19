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
    k:usize,
    s:Chars
        }
    let mut ans = vec![];
    let mut cur = 0;
    for i in 0..n {
        if cur >= k {
            ans.push('x');
            continue;
        }
        if s[i] == 'o' && cur < k {
            ans.push('o');
            cur += 1;
            continue;
        }
        ans.push('x');
    }
    println!("{}", ans.iter().join(""));
}
