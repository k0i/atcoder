use std::collections::VecDeque;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
       n:usize,
       a:[u64;n]
    }
    let mut res = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            res.push_back(a[i]);
        } else {
            res.push_front(a[i]);
        }
    }
    if n % 2 == 0 {
        for i in 0..n {
            if 0 < i {
                print!(" ");
            }
            print!("{}", res.pop_front().unwrap());
        }
    } else {
        for i in 0..n {
            if 0 < i {
                print!(" ");
            }
            print!("{}", res.pop_back().unwrap());
        }
    }
}
