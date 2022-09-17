use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    q:Chars,
        }
    let mut res = VecDeque::new();
    res.push_back(n);
    for i in (0..n).rev() {
        match q[i] {
            'R' => {
                res.push_front(i);
            }
            _ => {
                res.push_back(i);
            }
        }
    }
    for i in res {
        print!("{} ", i);
    }
}
