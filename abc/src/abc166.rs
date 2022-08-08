use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    x:i64
        }
    for a in -(10i64.pow(3))..10i64.pow(3) {
        for b in -(10i64.pow(3))..10i64.pow(3) {
            if a.pow(5) - b.pow(5) == x {
                println!("{} {}", a, b);
                return;
            }
        }
    }

    let mut res = 0;
    println!("{:?}", res);
}
