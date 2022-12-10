use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    for i in (0..=n).rev() {
        println!("{}", i);
    }
}
