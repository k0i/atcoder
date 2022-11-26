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
    }

    let mut cards = VecDeque::from(vec![1, 2, 3, 4, 5, 6]);
    let cnt = n / 5;
    let mo = n % 5;
    for i in 0..cnt {
        let first = cards.pop_front().unwrap();
        cards.push_back(first);
    }

    for i in 0..mo {
        cards.swap(i, i + 1);
    }
    println!("{}", cards.iter().join(""));
}
