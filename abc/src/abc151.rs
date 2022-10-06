use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,w:usize,
    s:[Chars;h],
        }
    let mut max = 0;

    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let mut q = std::collections::VecDeque::new();
            let mut dist = vec![vec![0; w]; h];
            let mut visited = vec![vec![false; w]; h];
            q.push_back((i, j));
            visited[i][j] = true;
            while let Some((x, y)) = q.pop_front() {
                for (dx, dy) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if s[nx][ny] == '#' || visited[nx][ny] {
                        continue;
                    }
                    visited[nx][ny] = true;
                    dist[nx][ny] = dist[x][y] + 1;
                    q.push_back((nx, ny));
                }
            }
            max = std::cmp::max(max, *dist.iter().flatten().max().unwrap());
        }
    }
    println!("{}", max);
}
