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
    m:usize,
    mut xy:[(U1,U1);m]
        }
    xy.sort();
    xy.dedup();
    let mut g = vec![Vec::new(); n];
    for (x, y) in xy {
        g[x].push(y);
    }
    let mut inv = vec![0; n];
    for i in 0..n {
        for &j in &g[i] {
            inv[j] += 1;
        }
    }
    let mut queue = Vec::new();
    for i in 0..n {
        if inv[i] == 0 {
            queue.push(i);
        }
    }
    if queue.len() == 0 || queue.len() > 1 {
        println!("No");
        return;
    }
    let mut toplogical_sort = Vec::new();
    while !queue.is_empty() {
        let v = queue.pop().unwrap();
        toplogical_sort.push(v);
        for &i in &g[v] {
            inv[i] -= 1;
            if inv[i] == 0 {
                queue.push(i);
            }
        }
        if queue.len() > 1 {
            println!("No");
            return;
        }
    }
    if toplogical_sort.len() != n {
        println!("No");
        return;
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        ans[toplogical_sort[i]] = i + 1;
    }
    println!("Yes");
    println!("{}", ans.iter().join(" "));
}
