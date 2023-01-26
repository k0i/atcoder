use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut m = HashMap::new();

    for i in 0..n {
        *m.entry(-a[i] + i as isize).or_insert(0isize) += 1;
    }

    let mut result = 0;

    for i in 0..n {
        if let Some(&value) = m.get(&(a[i] + i as isize)) {
            result += value;
        }
    }

    println!("{}", result);
}
fn d() {
    input! {
    x:i64
        }
    for a in -(10i64.pow(3))..10i64.pow(3) {
        for b in -(10i64.pow(3))..10i64.pow(3) {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }

    let mut res = 0;
    println!("{:?}", res);
}
