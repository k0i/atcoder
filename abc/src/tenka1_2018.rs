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
        n: usize,
        mut a: [usize;n],
    }
    a.sort();
    let mut b = a.clone();
    b.reverse();
    println!("{}", f(&a).max(f(&b)));
}

fn abs_diff(x: usize, y: usize) -> usize {
    x.max(y) - x.min(y)
}

fn diff_sum(b: &[usize]) -> usize {
    b.windows(2).map(|w| abs_diff(w[0], w[1])).sum::<usize>()
}

fn f(a: &[usize]) -> usize {
    let mut b = vec![];
    let mut it = a.iter();
    loop {
        match it.next() {
            Some(&x) => {
                b.push(x);
            }
            None => break,
        }
        match it.next_back() {
            Some(&x) => {
                b.push(x);
            }
            None => break,
        }
    }
    let mut sum = diff_sum(&b);
    sum -= abs_diff(b[b.len() - 2], b[b.len() - 1]);
    sum += abs_diff(b[0], b[b.len() - 1]);
    sum
}
