use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    x:i32,
    a:[usize;n]
        }
    let binary = format!("{:b}", x);
    let mut ans = 0;
    for i in 0..n {
        if binary.chars().rev().nth(i).unwrap_or('0') == '1' {
            ans += a[i];
        }
    }
    println!("{}", ans);
}
