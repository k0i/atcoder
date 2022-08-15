use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,
    a:[usize;n],
    }
    let nc3 = |n: i64| n * (n - 1) * (n - 2) / 6;
    let nc2 = |n: i64| n * (n - 1) / 2;
    let mut dup = HashMap::new();
    for i in a {
        let v = dup.entry(i).or_insert(0);
        *v += 1;
    }
    let mut res = nc3(n as i64);
    for (k, v) in dup.iter() {
        res = res - nc2(*v as i64) * ((n - v) as i64) - nc3(*v as i64);
    }
    println!("{:?}", res);
}
