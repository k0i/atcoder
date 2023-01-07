use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    t:usize
        }

    for i in 0..t {
        input! {
        n:usize,
        a:[usize;n]
                }
        let mut ans = 0;
        for i in a {
            if i % 2 != 0 {
                ans += 1;
            }
        }
        println!("{}", ans);
}
    }
}
