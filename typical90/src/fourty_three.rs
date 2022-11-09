use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        h: usize,
        w: usize,
        sr: Usize1,
        sc: Usize1,
        tr: Usize1,
        tc: Usize1,
        s: [String; h],
    }
    let wall = s
        .iter()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect::<Vec<Vec<bool>>>();
    let mut dist = vec![vec![vec![std::i32::MAX; 4]; w]; h];
    let mut que = VecDeque::new();
    for i in 0..4 {
        dist[sr][sc][i] = 0;
        que.push_back((sr, sc, i));
    }
    let dir = vec![(!0, 0), (0, 1), (1, 0), (0, !0)];
    while let Some((ur, uc, i)) = que.pop_front() {
        for j in 0..4 {
            let (dr, dc) = dir[i];
            let vr = ur.wrapping_add(dr);
            let vc = uc.wrapping_add(dc);
            if vr >= h || vc >= w || wall[vr][vc] {
                continue;
            }
            let cost = if i == j { 0 } else { 1 };
            if dist[vr][vc][j] > dist[ur][uc][i] + cost {
                dist[vr][vc][j] = dist[ur][uc][i] + cost;
                que.push_back((vr, vc, j));
            }
        }
    }

    let ans = dist[tr][tc].iter().min().unwrap();
    println!("{}", ans);
}
