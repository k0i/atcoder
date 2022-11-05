use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize
        }
    if h == 1 || w == 1 {
        println!("{}", h * w);
    } else {
        println!("{}", ((h + 1) / 2) * ((w + 1) / 2));
    }
}
