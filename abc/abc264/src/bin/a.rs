use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    l:usize,
    r:usize
    }
    let a: String = "atcoder".to_string().chars().take(r).skip(l - 1).collect();
    println!("{}", a);
}
