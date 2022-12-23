use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet, VecDeque};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    s:[Chars;h]
        }
    let mut black = 0;
    for i in 0..h {
        for j in 0..w {
            if let '#' = s[i][j] {
                black += 1;
            }
        }
    }
    let mut queue = VecDeque::new();
    let mut visited = vec![vec![-1; w]; h];
    queue.push_back((0, 0));
    visited[0][0] = 1;
    while let Some((i, j)) = queue.pop_front() {
        if i == h - 1 && j == w - 1 {
            println!("{}", (h * w - black) as i64 - visited[i][j]);
            return;
        }
        for (di, dj) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let ni = i as isize + di;
            let nj = j as isize + dj;
            if ni < 0 || ni >= h as isize || nj < 0 || nj >= w as isize {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if visited[ni][nj] != -1 {
                continue;
            }
            if let '#' = s[ni][nj] {
                continue;
            }
            visited[ni][nj] = visited[i][j] + 1;
            queue.push_back((ni, nj));
        }
    }
    println!("-1");
}
