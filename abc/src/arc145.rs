use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    s:Chars
        }
    if s[0] == 'A' && s[n - 1] == 'B' {
        println!("No");
        return;
    }
    if s[0] == 'B' && s[1] == 'A' && n == 2 {
        println!("No");
        return;
    }
    println!("Yes");
}
