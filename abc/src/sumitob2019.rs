use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: i64,
        s: String
    };

    let mut ans = 0;
    for i in 0..=999 {
        let mut t = i.to_string();
        if t.len() == 1 {
            t = String::from("00") + &t;
        } else if t.len() == 2 {
            t = String::from("0") + &t;
        }

        let t: Vec<_> = t.chars().collect();

        let mut i = 0;
        for c in s.chars() {
            if c == t[i] {
                i += 1;
            }

            if i == 3 {
                ans += 1;
                break;
            }
        }
    }

    println!("{}", ans);
}
