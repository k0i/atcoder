#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    m:usize,
    from_to:[(U1,U1);m],
        }
    let mut g = vec![vec![]; n];
    for (f, t) in from_to {
        g[f].push(t);
    }
    let mut ans = 0;
    for i in 0..n {
        let mut visited = vec![false; n];
        visited[i] = true;
        let mut queue = VecDeque::new();
        queue.push_back(i);
        while !queue.is_empty() {
            let first = queue.pop_front().unwrap();
            for &w in &g[first] {
                if !visited[w] {
                    visited[w] = true;
                    queue.push_back(w);
                    ans += 1;
                }
            }
        }
    }
    ans -= m;
    println!("{}", ans);
}
