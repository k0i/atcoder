#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    mut s:Chars
        }
    let ss = s.iter().map(|x| x.to_ascii_uppercase()).collect::<String>();
    println!("{}", ss);
}
