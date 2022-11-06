use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }

    let mut res = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            res = (i + 1) as i32;
        }
    }
    println!("{}", res);
}
