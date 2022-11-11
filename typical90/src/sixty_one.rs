use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let mut deck = VecDeque::new();

    for i in 0..n {
        input! {
        t:usize,
        x:usize
        }
        match t {
            1 => deck.push_front(x),
            2 => deck.push_back(x),
            _ => println!("{}", deck[x - 1]),
        }
    }
}
