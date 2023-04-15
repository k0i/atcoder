#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
        }

    let mut i = 1;
    let mut ans = BTreeSet::new();
    let mut sum = 0;
    while sum < n {
        sum += i;
        ans.insert(i);
        i += 1;
    }
    if sum == n {
        for i in ans {
            print!("{} ", i);
        }
        return;
    }
    let f = sum - n;
    ans.remove(&f);
    for i in ans {
        print!("{} ", i);
    }
}
