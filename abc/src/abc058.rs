use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    s:[Chars;n]
        }
    let mut cnt = vec![HashMap::new(); n];
    for i in 0..n {
        let mut tmp = HashMap::new();
        for j in s[i].iter() {
            *tmp.entry(j).or_insert(0) += 1;
        }
        cnt[i] = tmp;
    }
    let mut init = cnt[0].clone();
    for i in 1..n {
        let mut tmp = HashMap::new();
        for (k, v) in cnt[i].iter() {
            if init.contains_key(k) {
                tmp.insert(*k, std::cmp::min(init[k], *v));
            }
        }
        init = tmp;
    }
    let mut ans = vec![];
    for (k, v) in init.iter() {
        for _ in 0..*v {
            ans.push(*k);
        }
    }
    ans.sort();
    for i in ans.iter() {
        print!("{}", i);
    }
}
