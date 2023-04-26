#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    x:Chars
        }
    let mut ans = x.len();
    let mut stack = vec![];
    for i in 0..x.len() {
        if x[i] == 'S' {
            stack.push(x[i]);
        } else {
            if stack.len() > 0 && stack[stack.len() - 1] == 'S' {
                stack.pop();
                ans -= 2;
            } else {
                stack.push(x[i]);
            }
        }
    }
    println!("{}", ans);
}
