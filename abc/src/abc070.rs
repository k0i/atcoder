use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
    abc:[(Usize1,Usize1,usize);n-1],
    q:usize,
    k:Usize1,
    xy:[(Usize1,Usize1);q]
        }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        let (a, b, c) = abc[i];
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut from_k = vec![std::usize::MAX; n];
    from_k[k] = 0;
    dfs(k, &g, &mut from_k, 0);
    for i in 0..q {
        let (x, y) = xy[i];
        println!("{}", from_k[x] + from_k[y]);
    }
}

#[allow(dead_code)]
fn dfs(start: usize, graph: &[Vec<(usize, usize)>], dist: &mut [usize], current_dist: usize) {
    dist[start] = current_dist;
    for &(next, d) in &graph[start] {
        if dist[next] != std::usize::MAX {
            continue;
        }
        dfs(next, graph, dist, current_dist + d);
    }
}

fn c() {
    input! {
    n:usize,
    t:[usize;n]
        }
    let mut l = 1;
    for i in 0..n {
        l = lcm(t[i], l);
    }
    println!("{}", l);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a / g) * b
}
