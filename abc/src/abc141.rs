use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BinaryHeap, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut m: usize,
        va: [u64;n]
    }
    let mut queue = BinaryHeap::from(va);
    while m > 0 && !queue.is_empty() {
        let mut a = queue.pop().unwrap();
        a /= 2;
        m -= 1;
        if a > 0 {
            queue.push(a);
        }
    }
    let ans = queue.iter().fold(0, |acc, x| acc + x);
    println!("{}", ans);
}
