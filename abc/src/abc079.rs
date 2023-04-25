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
    h:usize,
    w:usize,
    c: [[usize;10];10],
    a: [[i64;w];h],
        }
    let mut shortest_path_to_one = vec![0; 10];
    for i in 0..10 {
        shortest_path_to_one[i] = c[i][1];
    }
    for i in 0..10 {
        if i == 1 {
            continue;
        }
        dfs(i, 0, &c, &mut shortest_path_to_one[i]);
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == -1 || a[i][j] == 1 {
                continue;
            }
            ans += shortest_path_to_one[a[i][j] as usize];
        }
    }
    println!("{}", ans);
}

fn dfs(start: usize, cost: usize, c: &Vec<Vec<usize>>, shortest_path_to_one: &mut usize) {
    for i in 0..10 {
        if i == start {
            continue;
        }
        let new_cost = cost + c[start][i];
        if new_cost < *shortest_path_to_one {
            if i == 1 {
                *shortest_path_to_one = new_cost;
            } else {
                dfs(i, new_cost, c, shortest_path_to_one);
            }
        }
    }
}
