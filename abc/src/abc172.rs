use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let mut ans = 0;
    for i in 1..n + 1 {
        let n = n / i;
        let cnt = (n * (2 * i + (n - 1) * i)) / 2;
        ans += cnt
    }
    println!("{}", ans)
}
