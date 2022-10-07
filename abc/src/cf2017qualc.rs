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
    let mut res = 0;
    let mut l = 0;
    let mut r = s.len() - 1;
    while l < r {
        let left = s[l];
        let right = s[r];
        if left == right {
            l += 1;
            r -= 1;
        } else if left == 'x' {
            res += 1;
            l += 1;
        } else if right == 'x' {
            res += 1;
            r -= 1;
        } else {
            println!("-1");
            return;
        }
    }
    println!("{}", res);
}
