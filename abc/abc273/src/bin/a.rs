use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize}
    let res = tes(n);
    println!("{}", res);
}
fn tes(a: usize) -> usize {
    if a == 0 {
        return 1;
    } else {
        return a * tes(a - 1);
    }
}
