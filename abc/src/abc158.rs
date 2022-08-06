use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {s:Chars,q:usize}
    let mut rev = false;
    let mut ve = VecDeque::from(s);
    for i in 0..q {
        input! {n:usize}
        if n == 1 {
            rev = !rev;
        } else {
            input! {f:usize,c:char}
            if !rev {
                if f == 1 {
                    ve.push_front(c);
                } else {
                    ve.push_back(c);
                }
            } else {
                if f == 1 {
                    ve.push_back(c);
                } else {
                    ve.push_front(c);
                }
            }
        }
    }
    if rev {
        for c in ve.iter().rev() {
            print!("{}", c);
        }
    } else {
        for c in ve {
            print!("{}", c);
        }
    }
}
