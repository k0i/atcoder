use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[fastout]
pub fn main() {
    input! {
        n: usize, m: usize, x: usize, y: usize,
        tap_list: [(usize, usize, usize, usize); m],
    }
    let mut graph = vec![vec![]; n + 1];
    for (a, b, t, k) in tap_list {
        graph[a].push((b, t, k));
        graph[b].push((a, t, k));
    }
    match dijkstra(x, y, n, graph) {
        None => println!("{}", -1),
        Some(n) => println!("{}", n),
    }
}

fn dijkstra(s: usize, e: usize, n: usize, graph: Vec<Vec<(usize, usize, usize)>>) -> Option<usize> {
    let mut q = BinaryHeap::new();
    q.push((Reverse(0), s));
    let mut visit = vec![std::usize::MAX; n + 1];
    visit[s] = 0;

    while let Some((Reverse(dist), cur)) = q.pop() {
        if visit[cur] < dist {
            continue;
        }
        for &(nxt, t, k) in &graph[cur] {
            let nxt_dist = dist + ((k - dist % k) % k) + t;
            if nxt_dist < visit[nxt] {
                visit[nxt] = nxt_dist;
                q.push((Reverse(nxt_dist), nxt));
            }
        }
    }
    return match visit[e] {
        std::usize::MAX => None,
        n => Some(n),
    };
}
