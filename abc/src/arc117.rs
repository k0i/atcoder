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
    let MOD: i64 = 1000000007;
    let mut ans: i64 = 1;

    a.push(0);
    a.sort();

    for i in 1..n + 1 {
        ans *= a[i] - a[i - 1] + 1;
        ans %= MOD;
    }

    println!("{}", ans);
}
