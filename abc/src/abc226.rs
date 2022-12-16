use std::collections::{HashSet, VecDeque};

use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[(i64,i64);n]
        }
    let mut cmb = a.into_iter().permutations(2).collect::<Vec<_>>();
    let mut hs = HashSet::new();
    for v in cmb {
        let (x1, y1) = v[0];
        let (x2, y2) = v[1];
        let dx = x2 - x1;
        let dy = y2 - y1;
        let g = gcd(dx, dy);
        if g == 0 {
            continue;
        }
        hs.insert((dx / g, dy / g));
        hs.insert((dx * -1 / g, dy * -1 / g));
    }
    println!("{}", hs.len());
}

fn gcd(x: i64, y: i64) -> i64 {
    if y == 0 {
        return x;
    }
    gcd(y, x % y)
}

fn c() {
    input! {
    n:usize
        }
    let mut skills = vec![];
    for _ in 0..n {
        input! {t:u64,k:usize,mut a:[usize;k]}
        a.sort();
        skills.push(Node {
            cost: t,
            next: a.to_vec(),
        });
    }
    println!("{}", Tree::new(n, skills).bfs());
}

#[derive(Clone, Debug)]
struct Node {
    cost: u64,
    next: Vec<usize>,
}
struct Tree {
    nodes: Vec<Node>,
    size: usize,
}

impl Tree {
    fn new(size: usize, nodes: Vec<Node>) -> Self {
        Self { size, nodes }
    }
    fn bfs(&self) -> u64 {
        let mut res = 0;
        let mut target = VecDeque::new();
        target.push_front(self.size - 1);
        let mut visited = vec![false; self.size];
        while !target.is_empty() {
            let nxt = target.pop_front().unwrap();
            if visited[nxt] {
                continue;
            }
            visited[nxt] = true;
            res += self.nodes[nxt].cost;
            self.nodes[nxt]
                .next
                .iter()
                .for_each(|n| target.push_front(n - 1));
        }
        res
    }
}
