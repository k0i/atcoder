use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    a.sort_by_key(|k| Reverse(*k));
    let ans: i64 = (0..n - 1).map(|i| a[(i + 1) / 2]).sum();
    println!("{}", ans);
}
fn c() {
    input! {
    h:usize,
    w:usize,
    k:usize,
    c:[Chars;h]
        }
    let mut res = 0;
    for i in 0..1 << h {
        let mut cc = c.clone();
        for j in 0..1 << w {
            let mut bl = 0;
            for t in 0..h {
                for y in 0..w {
                    if i >> t & 1 == 0 && j >> y & 1 == 0 && c[t][y] == '#' {
                        bl += 1;
                    }
                }
            }
            if bl == k {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
