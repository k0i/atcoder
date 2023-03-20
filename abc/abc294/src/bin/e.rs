#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    l:usize,
    no:usize,
    nt:usize,
    mut none:[(usize,usize);no],
    mut ntwo:[(usize,usize);nt],
        }
    let mut ans = run_length_same_counts(&mut none, &mut ntwo);
    println!("{}", ans);
}

// count same number of elements in two rl-compression. tuple is (value, count).
fn run_length_same_counts(
    first: &mut Vec<(usize, usize)>,
    second: &mut Vec<(usize, usize)>,
) -> usize {
    let mut ans = 0;
    let mut i = 0;
    let mut j = 0;
    loop {
        if i >= first.len() || j >= second.len() {
            break;
        }
        if first[i].0 == second[j].0 {
            ans += first[i].1.min(second[j].1);
            if first[i].1 > second[j].1 {
                first[i].1 -= second[j].1;
                j += 1;
            } else if first[i].1 < second[j].1 {
                second[j].1 -= first[i].1;
                i += 1;
            } else {
                i += 1;
                j += 1;
            }
        } else {
            let not_same_lenge = first[i].1.min(second[j].1);
            if first[i].1 > second[j].1 {
                first[i].1 -= not_same_lenge;
                j += 1;
            } else if first[i].1 < second[j].1 {
                second[j].1 -= not_same_lenge;
                i += 1;
            } else {
                i += 1;
                j += 1;
            }
        }
    }

    ans
}
