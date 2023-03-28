use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use superslice::Ext;
#[fastout]
pub fn main() {
    e()
}

fn e() {
    input! {
        n:usize,
        c:[usize;n],
        ab:[(Usize1,Usize1);n-1]
    }
    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        g[ab[i].0].push(ab[i].1);
        g[ab[i].1].push(ab[i].0);
    }
    let mut ans = BTreeSet::new();
    ans.insert(1);
    let mut colors = vec![0; 100001];
    colors[c[0]] = 1;
    let mut visited = vec![false; n];
    visited[0] = true;
    dfs(0, &g, &mut colors, &c, &mut visited, &mut ans);
    for &a in &ans {
        println!("{}", a);
    }
}

#[allow(dead_code)]
fn dfs(
    start: usize,
    graph: &[Vec<usize>],
    colors: &mut Vec<usize>,
    cs: &[usize],
    visited: &mut Vec<bool>,
    ans: &mut BTreeSet<usize>,
) {
    for &nv in &graph[start] {
        if visited[nv] {
            continue;
        }
        visited[nv] = true;
        if colors[cs[nv]] == 0 {
            ans.insert(nv + 1);
            colors[cs[nv]] = 1;
        } else {
            colors[cs[nv]] += 1;
        }
        dfs(nv, graph, colors, cs, visited, ans);
    }
    colors[cs[start]] -= 1;
}
fn is_ancestor(parent: usize, child: usize, in_time: &[usize], out_time: &[usize]) -> bool {
    in_time[parent] <= in_time[child] && out_time[child] <= out_time[parent]
}
fn is_descendant(parent: usize, child: usize, in_time: &[usize], out_time: &[usize]) -> bool {
    is_ancestor(child, parent, in_time, out_time)
}

fn d() {
    input! {
        s: [Bytes; 3],
    }
    let id = |c| (c - b'a') as usize;
    let mut n = 0;
    let mut c2i = vec![9999; 26];
    for t in &s {
        for &c in t {
            if c2i[id(c)] == 9999 {
                c2i[id(c)] = n;
                n += 1;
            }
        }
    }
    if n > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let mut p = (0..10).collect_vec();
    loop {
        let mut f = true;
        let mut x = vec![0i64; 3];
        for i in 0..3 {
            if p[c2i[id(s[i][0])]] == 0 {
                f = false;
                break;
            }
            for &c in &s[i] {
                x[i] = x[i] * 10 + p[c2i[id(c)]];
            }
        }
        if f && x[0] + x[1] == x[2] {
            for i in 0..3 {
                println!("{}", x[i]);
            }
            return;
        }
        if !p.next_permutation() {
            break;
        }
    }
    println!("UNSOLVABLE");
}

fn c() {
    input! {
    r:f64,
    x:f64,y:f64
        }
    let dis = (x * x + y * y).sqrt();
    let ans = (dis / r).ceil() as i64;
    if ans == 1 && dis != r {
        println!("2");
    } else {
        println!("{}", ans);
    }
}
