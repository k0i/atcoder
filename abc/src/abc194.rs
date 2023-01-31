#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    a:[usize;n]
        }
    let mut ans = std::usize::MAX;
    let mut used = vec![0; n + 1];
    for i in 0..m {
        used[a[i]] += 1;
    }
    for x in 0.. {
        if used[x] == 0 {
            ans = ans.min(x);
            break;
        }
    }
    for i in 0..n - m {
        used[a[i]] -= 1;
        used[a[i + m]] += 1;
        if used[a[i]] == 0 {
            ans = ans.min(a[i]);
        }
    }
    println!("{}", ans);
}
