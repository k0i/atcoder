use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    ab:[(usize,usize);n],
        }
    let mut max = BinaryHeap::new();
    for (a, b) in ab {
        max.push(b);
        max.push(a - b);
    }
    let mut ans = 0;
    for _ in 0..k {
        let x = max.pop().unwrap();
        ans += x;
    }
    println!("{}", ans);
}
