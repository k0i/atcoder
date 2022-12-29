use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BinaryHeap, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[i64;n],
    b:[i64;n],
        }
    let mut delta = 0;
    let mut ci = BinaryHeap::new();
    let mut ans = 0;
    for i in 0..n {
        ci.push(a[i] - b[i]);
        if a[i] < b[i] {
            delta += b[i] - a[i];
            ans += 1;
        }
    }
    while delta > 0 {
        if ci.is_empty() {
            println!("-1");
            return;
        }
        let c = ci.pop().unwrap();
        delta -= c;
        ans += 1;
    }
    println!("{}", ans);
}
