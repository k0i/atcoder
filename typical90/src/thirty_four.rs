use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n]
    }
    let mut hm = HashMap::new();
    let mut left = 0;
    let mut ans = 0;
    for right in 0..n {
        let v = hm.entry(a[right]).or_insert(0);
        *v += 1;
        while hm.len() > k {
            let v = hm.entry(a[left]).or_default();
            *v -= 1;
            if *v == 0 {
                hm.remove(&a[left]);
            }
            left += 1;
        }
        ans = std::cmp::max(ans, right + 1 - left);
    }
    println!("{}", ans);
}
