use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    mut  x:i64,
      y:i64
       }
    let ans = if x * y < 0 {
        (x + y).abs() + 1
    } else if x < y {
        y - x
    } else if x * y == 0 {
        x - y + 1
    } else {
        x - y + 2
    };

    println!("{}", ans);
}
