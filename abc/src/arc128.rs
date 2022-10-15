use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
        }
    let mut res = vec![0; n];
    for i in 0..n - 1 {
        if a[i] > a[i + 1] {
            res[i] ^= 1;
            res[i + 1] ^= 1;
        }
    }
    for i in res {
        print!("{} ", i);
    }
}
