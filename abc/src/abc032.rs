use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars,
    k:usize}

    if s.len() < k {
        println!("0");
        return;
    }
    let mut ans = HashSet::new();
    for i in 0..=s.len() - k {
        let str = s.iter().skip(i).take(k).join("");
        ans.insert(str);
    }
    println!("{}", ans.len());
}
