use std::collections::{BTreeMap, BTreeSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    q:usize
    }
    let mut t = MultiSet::new();

    for _ in 0..q {
        input! {
            x:usize
        }
        match x {
            1 => {
                input! {k:usize}
                t.insert(k)
            }
            2 => {
                input! {x:usize,k:Usize1}
                match t.lower_kth(x, k) {
                    Some((key, _)) => println!("{}", key),
                    None => println!("-1"),
                }
            }
            _ => {
                input! {x:usize,k:Usize1}
                match t.higer_kth(x, k) {
                    Some((key, _)) => println!("{}", key),
                    None => println!("-1"),
                }
            }
        }
    }
}

const INF: usize = 1 << 60;
#[derive(Debug)]
pub struct MultiSet<T>
where
    T: Ord,
{
    set: BTreeSet<(T, usize)>,
    index: usize,
}

impl<T> MultiSet<T>
where
    T: Ord + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            index: 0,
        }
    }

    pub fn lower_bound(&self, x: T) -> Option<usize> {
        let cnt = self.set.range(..(x, INF)).count();
        if cnt == self.set.len() {
            None
        } else {
            Some(cnt)
        }
    }

    pub fn lower_kth(&self, x: T, k: usize) -> Option<&(T, usize)> {
        self.set.range(..=(x, INF)).rev().nth(k)
    }

    pub fn higer_kth(&self, x: T, k: usize) -> Option<&(T, usize)> {
        self.set.range((x, 0)..).nth(k)
    }

    pub fn upper_bound(&self, x: T) -> Option<usize> {
        let cnt = self.set.range(..=(x, INF)).count();
        if cnt == self.set.len() {
            None
        } else {
            Some(cnt)
        }
    }

    pub fn insert(&mut self, x: T) {
        self.set.insert((x, self.index));
        self.index += 1;
    }

    pub fn is_empty(&self) -> bool {
        self.set.is_empty()
    }
}
