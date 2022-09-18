use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    a:i64,
    b:i64,
    c:i64,
    d:i64
        }
    let res = (a + b) * (c - d);
    println!("{:?}", res);
    println!("Takahashi");
}
