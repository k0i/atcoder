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
    day:usize,
    kind:usize,
    lr:[(Usize1,Usize1);day],
    sg:[(Usize1,Usize1);kind],
    }
    'outer: for i in 0..kind {
        let (start, goal) = sg[i];
        let mut curr = start;
        if start < goal {
            for j in 0..day {
                let (l, r) = lr[j];
                if l <= curr && curr <= r {
                    curr = r.min(goal);
                }
                if curr == goal {
                    println!("{}", j + 1);
                    continue 'outer;
                }
            }
        } else {
            for j in 0..day {
                let (l, r) = lr[j];
                if l <= curr && curr <= r {
                    curr = l.max(goal);
                }
                if curr == goal {
                    println!("{}", j + 1);
                    continue 'outer;
                }
            }
        }
    }
}
