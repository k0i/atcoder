use std::collections::{BTreeMap, HashMap, VecDeque};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
      n: usize, m: usize, k: usize,
      uva: [(Usize1, Usize1, usize); m],
      s: [Usize1; k],
    }

    let mut g = vec![vec![]; 2 * n];

    for (u, v, a) in uva {
        if a == 1 {
            g[u].push((v, 1));
            g[v].push((u, 1));
        } else {
            g[u + n].push((v + n, 1));
            g[v + n].push((u + n, 1));
        }
    }

    for x in s {
        g[x].push((x + n, 0));
        g[x + n].push((x, 0));
    }

    let mut que = VecDeque::new();
    que.push_back(0);

    let mut dist = vec![100100100; 2 * n];
    dist[0] = 0;

    while let Some(u) = que.pop_front() {
        for (v, c) in &g[u] {
            if dist[*v] <= dist[u] + c {
                continue;
            }
            dist[*v] = dist[u] + c;
            if *c == 0 {
                que.push_front(*v);
            } else {
                que.push_back(*v);
            }
        }
    }

    let mut ans = std::cmp::min(dist[n - 1], dist[2 * n - 1]);
    if ans == 100100100 {
        ans = -1;
    }
    println!("{}", ans);
}
fn d() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort_unstable();
    let sum: usize = a.iter().sum();
    a.append(&mut a.to_vec());

    let mut ans = sum;
    let mut cur = a[0];
    for i in 0..2 * n - 1 {
        if a[i + 1] == a[i] || a[i + 1] == (a[i] + 1) % m {
            cur += a[i + 1];
            continue;
        }
        if cur > sum {
            cur = sum;
        }
        ans = std::cmp::min(ans, sum - cur);
        cur = a[i + 1];
    }
    if cur >= sum {
        ans = 0;
    }
    println!("{}", ans);
}

fn c() {
    input! {
    n:usize,
    ab:[(Usize1,Usize1);n],
        }

    let mut tree = HashMap::new();
    for i in 0..n {
        let (a, b) = ab[i];
        tree.entry(a).or_insert(vec![]).push(b);
        tree.entry(b).or_insert(vec![]).push(a);
    }
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);
    let mut visited = vec![false; 1000000000];
    visited[0] = true;
    let mut ans = 1;
    while let Some(v) = queue.pop_front() {
        if tree.get(&v).is_none() {
            println!("{}", ans);
            return;
        }
        for &u in tree.get(&v).unwrap() {
            if visited[u] {
                continue;
            }
            visited[u] = true;
            queue.push_back(u);
        }
        ans = std::cmp::max(ans, v);
    }
    println!("{}", ans + 1);
}
