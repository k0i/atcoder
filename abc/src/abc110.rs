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
        s: Chars,
        t: Chars,
    }

    let mut st = HashMap::new();
    let mut ts = HashMap::new();

    let mut ans = true;
    for i in 0..s.len() {
        if st.entry(s[i]).or_insert(t[i]) != &t[i] {
            ans = false;
            break;
        }
        if ts.entry(t[i]).or_insert(s[i]) != &s[i] {
            ans = false;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
