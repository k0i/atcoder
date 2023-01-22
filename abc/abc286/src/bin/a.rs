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
    p:Usize1,
    q:Usize1,
    r:Usize1,
    s:Usize1,
    mut a:[usize;n]
        }
    let mut first = vec![];
    let mut second = vec![];
    for i in p..=q {
        first.push(a[i]);
    }
    for i in r..=s {
        second.push(a[i]);
    }
    let mut ans = vec![];
    let mut i = 0;
    while i < n {
        if i == p {
            ans.append(&mut second);
            i = q + 1;
        } else if i == r {
            ans.append(&mut first);
            i = s + 1;
        } else {
            ans.push(a[i]);
            i += 1;
        }
    }
    for i in 0..n - 1 {
        print!("{} ", ans[i]);
    }
    println!("{}", ans[n - 1]);
}
