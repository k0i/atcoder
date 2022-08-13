use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, VecDeque};
#[fastout]
pub fn main() {
    input! {s:Chars}
    if s == "atcoder".chars().collect::<Vec<char>>() {
        println!("0");
        return;
    }
    let mut visited = HashMap::new();
    let mut que = VecDeque::new();
    que.push_back(s.clone());
    visited.insert(s, 0);
    while !que.is_empty() {
        let cur = que.pop_front().unwrap();
        if cur == "atcoder".chars().collect::<Vec<_>>() {
            println!("{}", visited.get(&cur).unwrap());
            return;
        }
        for i in 1..cur.len() {
            let mut next = cur.clone();
            next.swap(i - 1, i);
            if !visited.contains_key(&next) {
                que.push_back(next.clone());
                visited.insert(next, visited.get(&cur).unwrap() + 1);
            }
        }
    }
}
