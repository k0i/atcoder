use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {h:usize,w:usize,a:usize,b:usize}
    for i in 0..h {
        for j in 0..w {
            if i < b {
                print!("{}", if j < a { 0 } else { 1 });
            } else {
                print!("{}", if j < a { 1 } else { 0 });
            }
        }
        println!("")
    }
}
