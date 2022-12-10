use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    mut d:usize,
    mut a:[usize;n]
        }
    a.sort();
    let mut set = HashSet::new();
    for i in a.iter() {
        set.insert(i);
    }

    let min = a.iter().take(k).sum::<usize>();
    let max = a.iter().rev().take(k).sum::<usize>();
    let mut i = 1;
    let mut d_multi = vec![d];
    while d * i <= max {
        d_multi.push(d * i);
        i += 1;
    }
    d_multi.reverse();
    let mut ans = -1;

    for t in d_multi {
        for i in a.iter() {
            for j in a.iter() {
                if i + j > t {
                    continue;
                }
                if set.contains(&(t - i - j)) {
                    ans = t as i64;
                }
            }
        }
    }

    println!("{}", ans);
}
