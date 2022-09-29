use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    q:usize,
    ab:[(usize,usize);n-1],
        }
    let mut tree = vec![vec![]; n + 1];
    let mut dis = vec![0; n + 1];
    for i in 0..n - 1 {
        let (a, b) = ab[i];
        tree[a].push(b);
        tree[b].push(a);
    }

    let mut queue = VecDeque::new();
    queue.push_back((1, 0));
    while let Some((node, d)) = queue.pop_front() {
        dis[node] = d;
        for &next in &tree[node] {
            if dis[next] == 0 {
                queue.push_back((next, d + 1));
            }
        }
    }
    for i in 0..q {
        input! {
            cd:[(usize,usize);1]
        }
        let (c, d) = cd[0];
        if (dis[c] + dis[d]) % 2 == 0 {
            println!("Town");
        } else {
            println!("Road");
        }
    }
}
