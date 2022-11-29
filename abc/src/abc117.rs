use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    mut a:[i64;m]
        }
    a.sort();
    let mut sub = vec![];
    for i in 0..m - 1 {
        sub.push(a[i + 1] - a[i]);
    }
    sub.sort();
    let mut i = std::cmp::max(m as i64 - n as i64, 0) as usize;
    let mut ans = 0;
    for j in 0..i {
        ans += sub[j];
    }
    println!("{}", ans);
}
