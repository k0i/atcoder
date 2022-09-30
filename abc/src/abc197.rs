use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n],
        }
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    let mut ans = std::usize::MAX;

    for i in 0..1 << n - 1 {
        let mut cur = a[0];
        let mut th = 0;
        for j in 1..n {
            if i & 1 << j - 1 != 0 {
                th ^= cur;
                cur = 0;
            }
            cur |= a[j];
        }
        th ^= cur;
        ans = std::cmp::min(ans, th);
    }
    println!("{}", ans);
}
