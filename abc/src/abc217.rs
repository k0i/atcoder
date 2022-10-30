use std::{
    cmp::Reverse,
    collections::{BTreeSet, BinaryHeap, VecDeque},
};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {q:usize}
    let mut deque = VecDeque::new();
    let mut queue = BinaryHeap::new();
    for i in 0..q {
        input! {x:usize}
        if x == 1 {
            input! {a:usize}
            deque.push_back(a);
            continue;
        }
        if x == 3 {
            for j in 0..deque.len() {
                let a = deque.pop_front();
                queue.push(Reverse(a.unwrap()));
            }
            continue;
        }
        if queue.is_empty() {
            let a = deque.pop_front();
            println!("{}", a.unwrap());
            continue;
        }
        let a = queue.pop();
        println!("{}", a.unwrap().0);
    }
}
fn d() {
    input! {
    l:usize,q:usize
        }

    let mut res = BTreeSet::new();
    res.insert(0);
    res.insert(l);

    for i in 0..q {
        input! {
        c:usize,x:usize
                }
        match c {
            1 => {
                res.insert(x);
            }
            _ => {
                let (a, b) = neighbors(&res, x);
                if let (Some(a), Some(b)) = (a, b) {
                    println!("{}", b - a);
                }
            }
        }
    }
}

fn neighbors(tree: &BTreeSet<usize>, val: usize) -> (Option<&usize>, Option<&usize>) {
    use std::ops::Bound::*;

    let mut before = tree.range((Unbounded, Excluded(val)));
    let mut after = tree.range((Excluded(val), Unbounded));

    (before.next_back(), after.next())
}
