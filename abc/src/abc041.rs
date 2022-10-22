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
    mut a:[usize;n]
        }
    let mut h = HashMap::new();
    for i in 0..n {
        let mut v = h.entry(a[i]).or_insert(i);
    }
    a.sort_by(|a, b| b.cmp(a));
    for i in a {
        let ind = h.get(&i).unwrap();
        print!("{} ", ind + 1);
    }
}
