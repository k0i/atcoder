use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    l:usize,
    mut s: [Chars;n]
        }
    s.sort();
    for i in 0..n {
        for j in 0..l {
            print!("{}", s[i][j]);
        }
    }
}
