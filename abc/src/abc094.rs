use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,
    x:[usize;n]}
    let mut s = x.clone();
    s.sort();
    for i in 0..n {
        if x[i] <= s[n / 2 - 1] {
            println!("{}", s[n / 2]);
        } else {
            println!("{}", s[n / 2 - 1]);
        }
    }
}
