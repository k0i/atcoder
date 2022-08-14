use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    c()
}

fn c() {
    input! {
    n:usize,
    s:[String;n]
    }
    let mut h = HashMap::new();
    for i in s {
        let cnt = h.entry(i.clone()).or_insert(0);
        if cnt == &0 {
            println!("{}", i);
        } else {
            println!("{}({})", i, cnt);
        }
        *cnt += 1;
    }
}
