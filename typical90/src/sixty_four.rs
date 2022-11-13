use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i64; n],
    }
    let mut ans = 0;
    let mut diff = vec![0; n - 1];
    for i in 0..n - 1 {
        diff[i] = a[i + 1] - a[i];
        ans += diff[i].abs();
    }
    for _ in 0..q {
        input! {
            l: Usize1,
            r: Usize1,
            v: i64,
        }
        if l > 0 {
            ans -= diff[l - 1].abs();
            diff[l - 1] += v;
            ans += diff[l - 1].abs();
        }
        if r + 1 < n {
            ans -= diff[r].abs();
            diff[r] -= v;
            ans += diff[r].abs();
        }
        println!("{}", ans);
    }
}
