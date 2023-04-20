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
    mut s:Chars
        }
    s.reverse();
    let mut target = HashSet::new();
    target.insert("maerd".to_string());
    target.insert("remaerd".to_string());
    target.insert("esare".to_string());
    target.insert("resare".to_string());

    let mut i = 0;
    while i < s.len() {
        let mut current = String::new();
        let mut j = i;
        loop {
            current.push(s[j]);
            if target.contains(&current) {
                break;
            }
            if j >= s.len() {
                println!("NO");
                return;
            }

            if current.len() > 7 {
                println!("NO");
                return;
            }
            j += 1;
        }
        i = j + 1;
    }
    println!("YES");
}
