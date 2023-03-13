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
fn c() {
    input! {
        a: usize,
        x: usize,
        m: usize,
    }

    let ans = f(a, x, m);

    println!("{}", ans);
}

fn f(a: usize, x: usize, m: usize) -> usize {
    if x == 0 {
        return 0;
    }

    if x % 2 == 0 {
        return (f(a, x / 2, m) * (1 + powmod(a, x / 2, m))) % m;
    } else {
        return (1 + a * f(a, x - 1, m)) % m;
    }
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
