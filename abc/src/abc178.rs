use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        s: usize,
    }

    let mut dp = vec![0; s + 1];
    let m = 1000000007;
    dp[0] = 1;

    for i in 3..=s {
        dp[i] = (dp[i - 1] + dp[i - 3]) % m;
    }
    println!("{}", dp[s]);
}
fn c() {
    input! {
    n:i64
    }
    let m = 10i64.pow(9) + 7;
    let ans = mp(10, n) - 2 * mp(9, n) + mp(8, n) + 2 * m;
    println!("{:?}", ans % m);
}

fn mp(x: i64, n: i64) -> i64 {
    let mut ans = 1;
    for _i in 0..n {
        ans = (ans * x) % (10i64.pow(9) + 7);
    }
    ans
}
