#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    q:usize
        }
    let n = 1048576;
    let mut s = BTreeSet::new();
    let mut v = vec![-1; n];

    for i in 0..n {
        s.insert(i);
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: usize,
        };

        let h = x % n;

        if t == 1 {
            let mut range = s.range(h..);
            let i = if let Some(i) = range.next() {
                *i
            } else {
                *s.range(..).next().unwrap()
            };

            v[i] = x as isize;
            s.remove(&i);
        } else {
            println!("{}", v[h]);
        }
    }
}
