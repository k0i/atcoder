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
    let mut div = HashSet::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            div.insert((i, n / i));
        }
        i += 1;
    }
    let mut ans = 1000000000000000000 as usize;
    for v in div {
        let (a, b) = v;
        let temp = std::cmp::max(a.to_string().len(), b.to_string().len());
        ans = std::cmp::min(ans, temp);
    }
    println!("{}", ans);
}
