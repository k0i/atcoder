use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    input! {
    n: usize,
    xy: [(i64, i64); n],
    }
    let mut pq = vec![];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            pq.push((xy[i].0 - xy[j].0, xy[i].1 - xy[j].1));
        }
    }

    let mut result = n;
    for &(p, q) in pq.iter() {
        let mut t = 0;
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                if (xy[i].0 - xy[j].0, xy[i].1 - xy[j].1) == (p, q) {
                    t += 1;
                }
            }
        }
        if n - t < result {
            result = n - t;
        }
    }
    println!("{}", result);
}
