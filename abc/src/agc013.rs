use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
            n:usize,
    a:[usize;n],
        }
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if i + 1 < n && a[i] == a[i + 1] {
            i += 1;
            continue;
        }

        if i + 1 < n && a[i] < a[i + 1] {
            while i + 1 < n && a[i] <= a[i + 1] {
                i += 1;
            }
        } else {
            while i + 1 < n && a[i] >= a[i + 1] {
                i += 1;
            }
        }
        ans += 1;
        i += 1;
    }
    println!("{}", ans);
}
