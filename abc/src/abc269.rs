use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    xy:[(i64,i64);n]
            }
    let coor = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];
    let mut t: Tree = vec![vec![]; n];
    for i in 0..n {
        let (ax, ay) = xy[i];
        for j in i + 1..n {
            let (bx, by) = xy[j];
            if coor.contains(&(ax - bx, ay - by)) {
                t[i].push(j);
                t[j].push(i);
            }
        }
    }

    let mut visited = vec![false; n];
    let mut ans = 0;
    for i in 0..n {
        if !visited[i] {
            dfs(&t, i, &mut visited);
            ans += 1;
        }
    }
    println!("{}", ans);
}

fn dfs(t: &Tree, i: usize, visited: &mut [bool]) {
    visited[i] = true;
    for &j in &t[i] {
        if !visited[j] {
            dfs(t, j, visited);
        }
    }
}

type Tree = Vec<Vec<usize>>;
