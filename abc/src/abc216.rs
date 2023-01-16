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
    n:usize,
    m:usize,
        }
    let mut graph = vec![vec![]; n];
    let mut cnt = vec![0; n];
    for _ in 0..m {
        input! {
        k:usize,
        a:[Usize1;k]
        }
        for i in 0..k - 1 {
            graph[a[i]].push(a[i + 1]);
            cnt[a[i + 1]] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in cnt.iter().enumerate() {
        if *i.1 == 0 {
            q.push_back(i.0);
        }
    }
    while !q.is_empty() {
        let v = q.pop_front().unwrap();
        for &i in &graph[v] {
            cnt[i] -= 1;
            if cnt[i] == 0 {
                q.push_back(i);
            }
        }
    }
    println!(
        "{}",
        if cnt.iter().all(|&i| i == 0) {
            "Yes"
        } else {
            "No"
        }
    );
}
