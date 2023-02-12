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
    mut a:[usize;m]
        }
    if m == 0 {
        println!("{}", (1..=n).join(" "));
        return;
    }
    a.sort();
    let mut ans = vec![];
    let mut i = 1;
    while i <= n {
        if !a.contains(&i) {
            ans.push(i);
            i += 1;
            continue;
        }
        let mut j = i;
        while j <= n && a.contains(&j) {
            j += 1;
        }
        for k in (i..=j).rev() {
            ans.push(k);
        }
        i = j + 1;
    }
    println!("{}", ans.iter().join(" "));
}
