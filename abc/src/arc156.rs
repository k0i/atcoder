#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input!(s: [(usize, String)]);

    for (_, s) in &s {
        let c = s.chars().filter(|&c| c == '1').count();
        let ans = if c == 2 && s.contains("11") {
            match s.len() {
                3 => None,
                4 if s == "0110" => Some(3),
                _ => Some(2),
            }
        } else if c % 2 == 0 {
            Some(c / 2)
        } else {
            None
        };
        if let Some(ans) = ans {
            println!("{}", ans);
        } else {
            println!("-1");
        }
    }
}
