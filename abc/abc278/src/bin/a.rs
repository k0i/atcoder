use std::collections::VecDeque;

use proconio::{fastout, input};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    a:[usize;n]
        }

    let mut b = VecDeque::from(a);

    for _ in 0..k {
        b.pop_front();
        b.push_back(0);
    }

    for i in b {
        print!("{} ", i);
    }
}
