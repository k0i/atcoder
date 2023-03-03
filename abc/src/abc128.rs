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
        n: usize,
        m: usize,
        s: [[Usize1]; m],
        p: [usize; m],
    }

    let mut result = 0;

    for mask in 0..1 << n {
        let mut ok = true;

        for i in 0..m {
            let mut count = 0;

            for &switch in s[i].iter() {
                if mask & 1 << switch > 0 {
                    count += 1;
                }
            }

            if count % 2 != p[i] {
                ok = false;
                break;
            }
        }

        if ok {
            result += 1;
        }
    }

    println!("{}", result);
}
