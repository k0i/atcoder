use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    a:usize,n:usize
        }
    let mut que = VecDeque::new();
    let mut max = 1;
    while max <= n {
        max *= 10;
    }
    let mut path = vec![-1; max];
    que.push_back(1);
    path[1] = 0;
    while !que.is_empty() {
        let cur = que.pop_front().unwrap();
        let cur_path = path[cur];

        let q1 = query1(a, cur);

        if q1 < max && path[q1] == -1 {
            que.push_back(q1);
            path[q1] = cur_path + 1;
        }
        let (b, q2) = query2(cur);
        if b && q2 < max && path[q2] == -1 {
            path[q2] = cur_path + 1;
            que.push_back(q2);
        }
    }
    println!("{}", path[n]);
}

fn query1(a: usize, i: usize) -> usize {
    i * a
}

fn query2(i: usize) -> (bool, usize) {
    let mut char_i: VecDeque<_> = i.to_string().chars().collect();
    if char_i.len() >= 2 {
        let last = char_i.pop_back().unwrap();
        if last != '0' {
            char_i.push_front(last);
            return (
                true,
                char_i.iter().collect::<String>().parse::<usize>().unwrap(),
            );
        }
    }
    (false, i)
}
