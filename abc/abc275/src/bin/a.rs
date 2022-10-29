use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    h:[i64;n],
        }
    let mut highest = 0;
    let mut higiest_v = 0;
    for i in 0..n {
        if higiest_v < h[i] {
            highest = i + 1;
            higiest_v = h[i];
        }
    }
    println!("{}", highest);
}
