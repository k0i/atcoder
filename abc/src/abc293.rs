#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {
    h:usize,
    w:usize,
    a:[[usize;w];h],
    }
    let mut ans = 0;
    for i in 0..1 << (h + w - 2) {
        let mut dup = HashSet::new();
        dup.insert(a[0][0]);
        let mut x = 0;
        let mut y = 0;
        for j in 0..h + w - 2 {
            if (i >> j) % 2 == 0 {
                x += 1;
            } else {
                y += 1;
            }
            if x >= h || y >= w {
                break;
            }
            if dup.contains(&a[x][y]) {
                break;
            }
            dup.insert(a[x][y]);
        }
        if dup.len() == h + w - 1 {
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn factorial(a: usize) -> usize {
    (1..=a).product()
}

fn d() {
    input! {
    n:usize,
    m:usize,
    abcd:[(Usize1,char,Usize1,char);m],
        }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        let (a, _, b, _) = abcd[i];
        g[a].push(b);
        g[b].push(a);
    }
    let mut uncycled = 0;
    let mut cycle = 0;
    let mut visited = vec![false; n];
    for i in 0..n {
        if visited[i] {
            continue;
        }
        let mut queue = vec![];
        queue.push(i);
        visited[i] = true;
        let mut edges = 0;
        let mut nodes = 0;
        while !queue.is_empty() {
            let v = queue.pop().unwrap();
            nodes += 1;
            for &w in &g[v] {
                edges += 1;
                if visited[w] {
                    continue;
                }
                visited[w] = true;
                queue.push(w);
            }
        }
        edges /= 2;
        if edges == nodes - 1 {
            uncycled += 1;
        } else {
            cycle += 1;
        }
    }
    println!("{} {}", cycle, uncycled);
}

fn powmod(a: usize, b: usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..64 {
        let w = 1 << i;
        if (b / w) % 2 == 1 {
            ans *= p;
            ans %= m;
        }
        p *= p;
        p %= m;
    }
    ans
}
