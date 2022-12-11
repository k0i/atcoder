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
    a:[usize;n]
        }
    let mut hm = HashMap::new();
    let mut ans = 0;
    for i in a.iter() {
        let v = hm.entry(*i).or_insert(0);
        *v += 1;
    }
    for (k, v) in hm.into_iter() {
        if k > v {
            ans += v;
        } else if k < v {
            ans += v - k;
        }
    }
    println!("{}", ans);
}
