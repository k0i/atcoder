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
        s: Chars,
    }
    let mut mn = 0;
    let mut cnt = 0;
    for i in 0..n {
        if s[i] == '(' {
            cnt += 1;
        } else {
            cnt -= 1;
            mn = mn.min(cnt);
        }
    }
    print!("{}", "(".repeat((-mn) as usize));
    print!("{}", s.iter().map(|x| x.to_string()).join(""));
    cnt += -mn;
    println!("{}", ")".repeat(cnt as usize));
}
