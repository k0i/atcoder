#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    input! {
    s:Chars,
      t:Chars
        }
    let mut r = s
        .clone()
        .into_iter()
        .rev()
        .take(t.len())
        .collect::<Vec<_>>();
    r.reverse();
    let mut right = HashMap::new();
    for i in 0..r.len() {
        right.insert(i, r[i]);
    }
    let mut diff = HashSet::new();
    for i in 0..t.len() {
        if right[&i] != t[i] && right[&i] != '?' && t[i] != '?' {
            diff.insert(i);
        }
    }
    if diff.len() > 0 {
        println!("No");
    } else {
        println!("Yes");
    }
    for i in 0..t.len() {
        diff.remove(&i);
        right.remove(&i);
        right.insert(i, s[i]);
        if right[&i] != t[i] && right[&i] != '?' && t[i] != '?' {
            diff.insert(i);
        }
        if diff.len() > 0 {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
