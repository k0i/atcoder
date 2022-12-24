use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    a:usize,
    b:usize
        }
    let ans = a.pow(b as u32);
    println!("{}", ans);
}
