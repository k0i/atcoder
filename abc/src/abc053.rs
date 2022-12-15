use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {mut n: usize};
    let mut res = 0;
    res += n / 11 * 2;
    n %= 11;
    res += n / 6;
    n %= 6;
    if n > 0 {
        res += 1;
    }
    println!("{}", res);
}
