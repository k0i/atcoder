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
    h:usize,w:usize,
    a:[Chars;h],
        }
    let mut black = vec![vec![false; w]; h];
    let mut queue = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == '#' {
                black[i][j] = true;
                queue.push_back((i, j));
            }
        }
    }

    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    let mut ans = 0;
    while !queue.is_empty() {
        let mut next = VecDeque::new();
        while let Some((i, j)) = queue.pop_front() {
            for k in 0..4 {
                let ni = i as isize + dx[k];
                let nj = j as isize + dy[k];
                if ni < 0 || nj < 0 || ni >= h as isize || nj >= w as isize {
                    continue;
                }
                let ni = ni as usize;
                let nj = nj as usize;
                if black[ni][nj] {
                    continue;
                }
                black[ni][nj] = true;
                next.push_back((ni, nj));
            }
        }
        ans += 1;
        queue = next;
    }
    println!("{}", ans - 1);
}
