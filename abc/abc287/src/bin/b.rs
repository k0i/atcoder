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
    m:usize,
    s:[Chars;n],
    t:[Chars;m]
        }
    let mut ss = vec![];
    for i in s {
        let mut last_three = i.into_iter().rev().take(3).collect::<Vec<_>>();
        last_three.reverse();
        ss.push(last_three);
    }
    let mut ans = 0;
    for i in ss {
        if t.contains(&i) {
            ans += 1;
        }
    }
    println!("{}", ans);
}
