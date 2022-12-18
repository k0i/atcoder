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
    mut b:[usize;n]
        }
    let mut ans = vec![];
    while !b.is_empty() {
        let mut flag = false;
        for i in (0..b.len()).rev() {
            if b[i] == i + 1 {
                ans.push(b.remove(i));
                flag = true;
                break;
            }
        }
        if !flag {
            println!("-1");
            return;
        }
    }
    ans.reverse();
    for i in ans {
        println!("{}", i);
    }
}
