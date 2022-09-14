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
    a:[usize;n]
        }
    let mut res = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            res.push_back(a[i]);
        } else {
            res.push_front(a[i]);
        }
    }
    if n % 2 == 1 {
        for i in (0..n).rev() {
            println!("{} ", res[i]);
        }
        return;
    }
    for i in 0..n {
        println!("{} ", res[i]);
    }
}
