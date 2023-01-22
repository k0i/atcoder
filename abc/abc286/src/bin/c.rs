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
    a:usize,
    b:usize,
    mut s:Chars
        }
    let mut ans = std::u64::MAX;
    let mut cnt = 0;
    for i in 0..n {
        let mut palindrome_pair = vec![];
        for j in 0..n / 2 {
            palindrome_pair.push((s[j], s[n - j - 1]));
        }
        let mut temp = 0;
        for i in 0..n / 2 {
            if palindrome_pair[i].0 != palindrome_pair[i].1 {
                temp += 1;
            }
        }
        ans = std::cmp::min(ans, (temp * b + cnt * a) as u64);
        let aaa = s.remove(0);
        s.push(aaa);
        cnt += 1;
    }
    println!("{}", ans);
}
