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
    n:usize,
    d:isize,
    xy:[(f64,f64);n]
        }
    let mut visited = vec![false; n];
    visited[0] = true;
    let mut queue = vec![];
    queue.push(0);
    while !queue.is_empty() {
        let current = queue.pop().unwrap();
        for i in 0..n {
            if visited[i] {
                continue;
            }
            let (x1, y1) = xy[current];
            let (x2, y2) = xy[i];
            let dist = ((x1 - x2).powf(2.0) + (y1 - y2).powf(2.0)).sqrt();
            if dist <= d as f64 {
                visited[i] = true;
                queue.push(i);
            }
        }
    }
    for i in 0..n {
        if !visited[i] {
            println!("No");
            continue;
        }
        println!("Yes");
    }
}
