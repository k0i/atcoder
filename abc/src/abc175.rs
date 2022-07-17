use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut x:i128,
     k:i128,
     d:i128
         }
    x = x.abs();
    let mov = x / d;
    let pred = x - mov * d;
    if mov > k {
        println!("{}", (x - k * d).abs());
        return;
    }
    let other = k - mov;
    if other % 2 == 0 {
        println!("{}", pred.abs());
    } else {
        println!("{}", (pred - d).abs());
    }
}
