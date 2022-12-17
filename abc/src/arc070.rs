use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    x:usize
        }
    let mut i = 1;
    let mut sec = 0;
    while sec < x {
        sec += i;
        i += 1;
    }
    println!("{}", i - 1);
}
