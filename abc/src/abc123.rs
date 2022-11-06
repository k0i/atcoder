use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;5],
        }
    let mut min = std::usize::MAX;
    for i in a {
        if min > i {
            min = i;
        }
    }
    let ans = 4 + (n + min - 1) / min;
    println!("{}", ans);
}
