use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,k:usize}
    let mut res = 1;
    res *= k;
    if n == 1 {
        println!("{}", res);
        return;
    }
    for i in 1..n {
        res *= k - 1;
    }
    println!("{}", res);
}
