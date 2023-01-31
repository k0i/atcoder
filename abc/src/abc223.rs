#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

pub fn main() {
    d();
}

fn d() {
    input! {
        n: usize,
        m: usize,
    }
    let mut g = vec![vec![]; n];
    let mut cnt = vec![0; n];
    for _ in 0..m {
        input! {
            a: Usize1,
            b: Usize1,
        }
        g[a].push(b);
        cnt[b] += 1;
    }
    let mut pq = BinaryHeap::new();
    for i in 0..n {
        if cnt[i] == 0 {
            pq.push(Reverse(i));
        }
    }
    let mut res = vec![];
    while pq.len() > 0 {
        let v = pq.pop().unwrap().0;
        res.push(v + 1);
        for &u in &g[v] {
            cnt[u] -= 1;
            if cnt[u] == 0 {
                pq.push(Reverse(u));
            }
        }
    }
    if res.len() < n {
        println!("-1");
    } else {
        println!("{}", res.iter().map(|x| x.to_string()).join(" "));
    }
}

fn c() {
    input! {
    n:usize,
    d:[(f64,f64);n]
        }
    let mut time = 0.0;
    d.iter().for_each(|&(a, b)| {
        time += a / b;
    });
    time /= 2.0;
    let mut ans = 0.0;
    for i in 0..n {
        let t = d[i].0 / d[i].1;
        if t < time {
            time -= t;
            ans += d[i].0;
        } else {
            ans += d[i].1 * time;
            break;
        }
    }
    println!("{}", ans);
}
