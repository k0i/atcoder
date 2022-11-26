use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    a:f64,
      b:f64
        }
    let mut right = a as usize;
    let mut left = 0;
    let mut ans = a;
    while right - left > 1 {
        let mid = ((right + left) / 2) as f64;
        if b * (mid - 1.) + a / mid.sqrt() < b * (mid) + a / (mid + 1.).sqrt() {
            right = mid as usize;
        } else {
            left = mid as usize;
        }
    }
    ans = ans.min(b * (right as f64 - 1.) + a / (right as f64).sqrt());
    println!("{}", ans);
}
