#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
        }
    let mut divs = HashMap::new();
    for i in 1..=n {
        let mut j = 1;
        while j * j <= i {
            if i % j == 0 {
                *divs.entry(i).or_insert(0) += 1;
                if j != i / j {
                    *divs.entry(i).or_insert(0) += 1;
                }
            }
            j += 1;
        }
    }
    let mut ans = 0;

    for i in 1..=n {
        let v = divs.get(&i).unwrap();
        let kk = n - i;
        let v2 = divs.get(&kk).unwrap_or(&0);
        ans += v * v2;
    }
    println!("{}", ans);
}
