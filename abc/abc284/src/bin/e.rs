use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
fn dfs(
    mut visited: Vec<bool>,
    graph: &Vec<Vec<usize>>,
    n: usize,
    mut count: usize,
) -> (Vec<bool>, usize) {
    if count == 1_000_000 {
        return (visited, count);
    }

    visited[n] = true;
    count += 1;

    for &x in graph[n].iter() {
        if visited[x] {
            continue;
        };
        let (v, c) = dfs(visited, graph, x, count);
        visited = v;
        count = c;
    }
    visited[n] = false;

    (visited, count)
}

pub fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let visited = vec![false; n];
    let mut graph: Vec<Vec<usize>> = vec![vec![]; n];

    for _ in 0..m {
        input! {
            mut u: Usize1,
            mut v: Usize1,
        };
        graph[u].push(v);
        graph[v].push(u);
    }
    let (_, ans) = dfs(visited, &graph, 0, 0);

    println!("{}", ans);
}
