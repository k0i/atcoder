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
    s:Chars
        }
    let mut ans = "No";
    let mut b_pos = vec![];
    for i in 0..s.len() {
        if s[i] == 'B' {
            b_pos.push(i);
        }
    }
    if b_pos[0] % 2 == 0 {
        if b_pos[1] % 2 == 0 {
            println!("No");
            return;
        }
    } else {
        if b_pos[1] % 2 == 1 {
            println!("No");
            return;
        }
    }
    let mut k_pos = 0;
    let mut r_pos = vec![];
    for i in 0..s.len() {
        if s[i] == 'R' {
            r_pos.push(i);
        }
        if s[i] == 'K' {
            k_pos = i;
        }
    }
    if r_pos[0] < k_pos && k_pos < r_pos[1] {
        ans = "Yes";
    }
    println!("{}", ans);
}
