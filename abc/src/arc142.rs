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
    mut k:Chars
        }
    let mut res = 0;
    let mut a = k.iter().collect::<String>().parse::<usize>().unwrap();
    k.reverse();
    let mut b = k.iter().collect::<String>().parse::<usize>().unwrap();

    if a > b {
        println!("0");
        return;
    }
    let mut dedup = HashSet::new();
    while a <= n {
        if dedup.contains(&a) {
            a *= 10;
            continue;
        }
        res += 1;
        dedup.insert(a);
        a *= 10;
    }
    while b <= n {
        if dedup.contains(&b) {
            b *= 10;
            continue;
        }
        res += 1;
        dedup.insert(b);
        b *= 10;
    }
    println!("{}", res);
}
