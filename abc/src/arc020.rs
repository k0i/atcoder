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
    c:usize,
    a:[usize;n],
        }
    let mut ans = n * c;
    let mut color_perm = (1..=10).permutations(2).collect::<Vec<_>>();
    for perm in color_perm {
        let mut cost = 0;
        for i in 0..n {
            if i % 2 == 0 {
                if a[i] != perm[0] {
                    cost += c;
                }
            } else {
                if a[i] != perm[1] {
                    cost += c;
                }
            }
        }
        ans = std::cmp::min(ans, cost);
    }
    println!("{}", ans);
}
