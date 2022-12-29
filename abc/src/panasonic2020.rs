use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let d = c - a - b;
    if d > 0 && 4 * a * b < d * d {
        println!("Yes");
    } else {
        println!("No");
    }
}
