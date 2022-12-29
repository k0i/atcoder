use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        s: Chars
    }

    let count = s.into_iter().fold([0; 26], |mut count, c| {
        count[c as usize - 0x61] += 1;
        count
    });

    let mut odd = 0;
    let mut even = 0;
    for c in count.iter() {
        if c % 2 == 1 {
            odd += 1;
        }
        even += c / 2
    }

    if odd == 0 {
        println!("{}", even * 2);
    } else {
        println!("{}", 1 + (even / odd) * 2);
    }
}
