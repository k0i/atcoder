use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    a:usize,
    b:usize,
    n:usize
        }

    let c = |a: usize, b: usize, x: usize| -> usize { (a * x) / b - a * (x / b) };
    let mut ans = 0;
    ans = std::cmp::max(ans, c(a, b, n));
    if b - 1 <= n {
        ans = std::cmp::max(ans, c(a, b, b - 1));
    }

    println!("{}", ans);
}
