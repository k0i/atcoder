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
    let mut res = 0;
    let mut a = 1;
    while a * a * a <= n {
        for b in a..=n {
            if a * b * b > n {
                break;
            }
            let ab = a * b;
            let c = n / ab;
            res += c - b + 1;
        }
        a += 1;
    }
    println!("{}", res);
}
