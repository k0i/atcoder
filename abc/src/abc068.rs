use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n: usize,
    m: usize,
    a: [(Usize1,Usize1); m],
        }
    let mut con = vec![vec![]; n];
    for i in 0..m {
        let (x, y) = a[i];
        con[x].push(y);
        con[y].push(x);
    }
    let mut dist = vec![-1; n];

    let mut queue = VecDeque::new();
    queue.push_front(0);

    dist[0] = 0;

    while !&queue.is_empty() {
        let v = queue.pop_back().unwrap();

        for nv in con[v].iter() {
            if dist[*nv] > -1 {
                continue;
            }
            dist[*nv] = dist[v] + 1;
            queue.push_front(*nv);
        }
    }

    if dist[n - 1] > -1 && dist[n - 1] <= 2 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
