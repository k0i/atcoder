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
    k:i64,
    a:[i64;n]
        }
    let f = |x: i64| {
        let mut ret = 0;
        for i in 0..n {
            ret += (a[i] - x).max(0);
        }
        ret <= k
    };
    let mut l = -1;
    let mut r = 10000000000;
    while l + 1 != r {
        let mid = (l + r) / 2;
        if f(mid) {
            r = mid;
        } else {
            l = mid;
        }
    }
    let mut ans = 0;
    let mut count = 0;
    for i in 0..n {
        let c = (a[i] - r).max(0);
        count += c;
        ans += a[i] * c;
        ans -= c * (c - 1) / 2;
    }
    ans += r * (k - count);
    println!("{}", ans);
}
fn d() {
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
