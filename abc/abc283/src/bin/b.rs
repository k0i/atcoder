use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[usize;n],
    q:usize
    }

    for i in 0..q {
        input! {
        x:usize
        }
        match x {
            1 => {
                input! {
                k:usize,
                y:usize
                }
                a[k - 1] = y;
            }
            _ => {
                input! {k:usize}
                println!("{}", a[k - 1]);
            }
        }
    }
}
