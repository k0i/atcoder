#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize
        }
    let mut c = vec![];
    for i in 0..m {
        input! {
            x:usize,
            mut a:[usize;x]
        }
        let mut b = HashSet::new();
        for i in a {
            b.insert(i);
        }
        c.push(b);
    }
    let mut ans = 0;
    for i in 0..1 << m {
        let mut d = HashSet::new();
        for j in 0..m {
            if i >> j & 1 == 1 {
                for k in &c[j] {
                    d.insert(k);
                }
            }
        }
        for i in 1..=n {
            if !d.contains(&i) {
                break;
            }
            if i == n {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
