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
    s:[String;n]
    }
    let mut vote = HashMap::new();
    for i in s {
        let v = vote.entry(i).or_insert(0);
        *v += 1;
    }

    let mut ans = 0;
    let mut name = String::new();
    for (k, v) in vote {
        if ans < v {
            ans = v;
            name = k;
        }
    }
    println!("{}", name);
}
