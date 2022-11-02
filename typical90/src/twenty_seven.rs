use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    s:[Chars;n]
        }
    let mut h = HashMap::new();
    for i in 0..n {
        if !h.contains_key(&s[i]) {
            println!("{}", i + 1);
            h.insert(s[i].clone(), i);
        }
    }
}
