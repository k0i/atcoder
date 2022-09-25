use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
      n:usize,
      mut k:usize,
    mut  a:[usize;n]
          }
    let mut right = (10u64.pow(12) + 2) as usize;
    let mut left = 0;

    while right - left > 1 {
        let mid = (right + left) / 2;
        let mut cnt = 0;
        for i in &a {
            cnt += std::cmp::min(mid, *i);
        }
        if cnt <= k {
            left = mid;
        } else {
            right = mid;
        }
    }
    for i in 0..n {
        let temp = std::cmp::min(left, a[i]);
        a[i] -= temp;
        k -= temp;
    }
    let mut i = 0;
    while k > 0 {
        if a[i] > 0 {
            a[i] -= 1;
            k -= 1;
        }
        i += 1;
    }
    for i in a {
        print!("{} ", i);
    }
}
