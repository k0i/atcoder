use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {a:usize,b:usize,x:usize}
    if a == 0 {
        println!("{}", div(b, x) + 1);
    } else {
        println!("{}", div(b, x) - div(a - 1, x));
    }
}

fn div(n: usize, x: usize) -> usize {
    n / x
}
