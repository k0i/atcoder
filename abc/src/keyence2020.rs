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
    s:usize,
        }
    let mut ans = vec![];
    if s < 1000000000 {
        for i in 0..k {
            ans.push(s);
        }
        for i in 0..n - k {
            ans.push(1000000000);
        }
    } else {
        for i in 0..k {
            ans.push(s);
        }
        for i in 0..n - k {
            ans.push(1);
        }
    }
    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
