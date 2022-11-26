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
    let mut ans = 0;
    for i in s {
        if i == 'v' {
            ans += 1;
        } else {
            ans += 2;
        }
    }
    println!("{:?}", ans);
}
