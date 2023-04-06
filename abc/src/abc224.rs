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
        m: usize,
    }
    let mut g = vec![vec![]; 9];
    for i in 0..m {
        input! {
            u: Usize1,
            v: Usize1,
        }
        g[u].push(v);
        g[v].push(u);
    }
    let mut p = vec![8; 9];
    for i in 0..8 {
        input! {
            pi: Usize1,
        }
        p[pi] = i;
    }
    let mut dist = HashMap::new();
    let mut q = VecDeque::new();
    dist.insert(p.clone(), 0);
    q.push_back(p.clone());
    while q.len() > 0 {
        let v = q.pop_front().unwrap();
        let d = *dist.get(&v).unwrap();
        let z = v.iter().position(|&x| x == 8).unwrap();
        for &e in &g[z] {
            let mut nv = v.clone();
            nv.swap(e, z);
            if !dist.contains_key(&nv) {
                dist.insert(nv.clone(), d + 1);
                q.push_back(nv);
            }
        }
    }
    let gl = (0..9).collect_vec();
    println!("{}", dist.get(&gl).unwrap_or(&-1));
}
