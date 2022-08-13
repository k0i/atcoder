use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,m:usize,r:[(usize,usize);m]}
    let mut con = vec![vec![]; n + 1];
    for (a, b) in r {
        con[a].push(b);
        con[b].push(a);
    }
    let mut visited = vec![false; n + 1];
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut res = vec![0; n + 1];
    while !que.is_empty() {
        let v = que.pop_front().unwrap();
        for i in con[v].iter() {
            if !visited[*i] {
                visited[*i] = true;
                que.push_back(*i);
                res[*i] = v;
            }
        }
    }
    println!("Yes");
    for i in 2..n + 1 {
        println!("{}", res[i]);
    }
}
