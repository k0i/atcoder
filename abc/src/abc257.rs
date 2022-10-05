use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    xy:[(i64,i64,i64);n]
        }
    let mut left = -1;
    let mut right = 4000000001;
    let movable = |(x, y): (i64, i64), (x2, y2): (i64, i64), s: i64, p: i64| -> bool {
        s * p >= (x - x2).abs() + (y - y2).abs()
    };
    let bfs = |m: i64| -> bool {
        for start in 0..n {
            let mut queue = std::collections::VecDeque::new();
            let mut visited = vec![false; n];
            queue.push_back(start);
            visited[start] = true;
            while let Some(i) = queue.pop_front() {
                for j in 0..n {
                    if !visited[j] && movable((xy[i].0, xy[i].1), (xy[j].0, xy[j].1), m, xy[i].2) {
                        queue.push_back(j);
                        visited[j] = true;
                    }
                }
            }
            if visited.iter().all(|&x| x) {
                return true;
            };
        }
        false
    };
    while left + 1 != right {
        let mid = (left + right) / 2;
        if bfs(mid) {
            right = mid;
        } else {
            left = mid;
        }
    }
    println!("{}", right);
}
