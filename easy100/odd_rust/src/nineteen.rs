use proconio::input;
use std::cmp::min;
pub fn main() {
    input! {
          _: usize,
          m:usize,
          x:usize,
          a: [usize; m]
    }
    let mut left = 0;
    let mut right = 0;

    a.iter().for_each(|&i| {
        if i < x {
            left += 1
        } else {
            right += 1;
        }
    });
    println!("{}", min(left, right));
}