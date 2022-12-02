use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[i64;n],
        }
    a.sort();
    let mut ans = 0;
    let mut cmsm = vec![0; n + 1];
    for i in 0..n {
        cmsm[i + 1] = cmsm[i] + a[i];
    }
    for i in 0..n {
        ans += (a[i] * (n - i - 1) as i64 - cmsm[n] + cmsm[i + 1]).abs();
    }
    println!("{}", ans);
}
