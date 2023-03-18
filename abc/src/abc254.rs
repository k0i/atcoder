use std::collections::{HashMap, HashSet, VecDeque};

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    e()
}
fn e() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        q: usize,
        xk: [(Usize1, usize); q],
    }

    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }

    for (x, k) in xk {
        let mut sum = 0;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        q.push_back((x, 0));
        visited.insert(x);
        sum += x + 1;
        while let Some((node, i)) = q.pop_front() {
            for &next in &g[node] {
                if visited.contains(&next) || i + 1 > k {
                    continue;
                }
                visited.insert(next);
                q.push_back((next, i + 1));
                sum += next + 1;
            }
        }
        println!("{}", sum);
    }
}

#[allow(dead_code)]
fn bfs(start: usize, graph: &[Vec<usize>]) -> (Vec<usize>, Vec<usize>) {
    let mut dist = vec![std::usize::MAX; graph.len()];
    let mut parents = vec![std::usize::MAX; graph.len()];
    let mut que = std::collections::VecDeque::new();
    dist[start] = 0;
    que.push_back(start);
    while let Some(v) = que.pop_front() {
        for &nv in &graph[v] {
            if dist[nv] != std::usize::MAX {
                continue;
            }
            dist[nv] = dist[v] + 1;
            parents[nv] = v;
            que.push_back(nv);
        }
    }
    (dist, parents)
}
fn reconstruct_path(start: usize, goal: usize, parents: &[usize]) -> Vec<usize> {
    let mut path = vec![goal];
    let mut cur = goal;
    while cur != start {
        cur = parents[cur];
        path.push(cur);
    }
    path.reverse();
    path
}
fn c() {
    input! {n:usize,k:usize,mut a:[u64;n]}
    let mut sorted = vec![vec![]; k];

    for i in 0..n {
        let idx = i % k;
        sorted[idx].push(a[i]);
    }
    for i in 0..k {
        sorted[i].sort();
    }
    let mut b = a;
    b.sort();
    for i in 0..n {
        let idx = i % k;
        if b[i] != sorted[idx][i / k] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn b() {
    input! {n:usize}
    let mut res = vec![vec![]];
    for i in 0..n {
        if i == 0 {
            res[0].push(1);
            continue;
        }
        let mut tmp = vec![];
        for j in 0..i + 1 {
            if j == 0 || j == i {
                tmp.push(1);
            } else {
                tmp.push(res[i - 1][j - 1] + res[i - 1][j]);
            }
        }
        res.push(tmp);
    }
    for i in res {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}
fn a() {
    input! {
    n:u64
        }
    let a = n.to_string().chars().rev().take(2).collect::<String>();
    let b = a.to_string().chars().rev().collect::<String>();
    println!("{}", b);
}
