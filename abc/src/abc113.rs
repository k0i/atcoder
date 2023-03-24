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
    py_:[(usize,usize);m],
        }
    let mut py = vec![(0, 0, 0)];
    let mut answer = vec![vec![]; m];
    for i in 0..m {
        py.push((py_[i].0, py_[i].1, i));
    }
    py.push((1_000_000, 0, 0));
    py.sort();
    let mut num = 0;
    for i in 1..=m {
        answer[py[i].2].push(py[i].0);
        if py[i].0 != py[i - 1].0 {
            num = 0;
        }
        num += 1;
        answer[py[i].2].push(num);
    }

    for ans in answer {
        println!("{:0>6}{:0>6}", ans[0], ans[1]);
    }
}
