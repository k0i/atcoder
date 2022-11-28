use std::collections::{LinkedList, VecDeque};

use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    let INF = std::usize::MAX;
    input!(n: usize, q: usize);
    let mut back = vec![INF; n];
    let mut front = vec![INF; n];

    for _ in 0..q {
        input!(t: usize);
        if t == 1 {
            input!(x: Usize1, y: Usize1);
            back[x] = y;
            front[y] = x;
        } else if t == 2 {
            input!(x: Usize1, y: Usize1);
            back[x] = INF;
            front[y] = INF;
        } else {
            input!(x: Usize1);
            let mut ans = VecDeque::new();
            ans.push_back(x);
            let mut cur = x;
            while front[cur] != INF {
                let nxt = front[cur];
                ans.push_front(nxt);
                cur = nxt;
            }
            let mut cur = x;
            while back[cur] != INF {
                let nxt = back[cur];
                ans.push_back(nxt);
                cur = nxt;
            }
            let l = ans.len();
            print!("{} ", l);
            for (i, v) in ans.iter().enumerate() {
                if i != ans.len() - 1 {
                    print!("{} ", v + 1);
                } else {
                    println!("{}", v + 1);
                }
            }
        }
    }
}
