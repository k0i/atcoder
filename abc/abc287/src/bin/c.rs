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
    uv:[(Usize1,Usize1);m]
        }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (u, v) = uv[i];
        g[u].push(v);
        g[v].push(u);
    }
    let mut one = 0;
    let mut two = 0;
    for i in 0..n {
        if g[i].len() > 2 {
            println!("No");
            return;
        }
        if g[i].len() == 1 {
            one += 1;
        }
        if g[i].len() == 2 {
            two += 1;
        }
    }
    if one != 2 || two != n - 2 {
        println!("No");
        return;
    }
    let mut queue = vec![];
    let mut visited = vec![false; n];
    let mut path = vec![];
    queue.push(0);
    path.push(0);
    visited[0] = true;
    while queue.len() > 0 {
        let v = queue.pop().unwrap();
        for i in g[v].clone() {
            if !visited[i] {
                queue.push(i);
                path.push(i);
                visited[i] = true;
            }
        }
    }
    if path.len() == n {
        println!("Yes");
    } else {
        println!("No");
    }
}
