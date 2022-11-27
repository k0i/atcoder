use std::collections::HashSet;

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

    let mut ans = 0;
    let mut hs = HashSet::new();
    for i in a {
        if hs.contains(&i) {
            ans += 1;
            continue;
        }
        hs.insert(i);
    }
    println!("{}", ans);
}
