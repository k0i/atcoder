use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut x:u64,
    k:usize
        }
    let mut div = 1;
    for i in 0..k {
        x /= div;
        let x_digits = x
            .to_string()
            .chars()
            .map(|y| y.to_digit(10).unwrap())
            .collect_vec();
        let last = x_digits[x_digits.len() - 1] as u64;
        if last >= 5 {
            x += 10 - last;
        } else {
            x -= last;
        }
        x *= div;
        div *= 10;
    }
    println!("{}", x);
}
