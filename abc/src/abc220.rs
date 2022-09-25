use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
        }
    let MOD = 998244353;
    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..10 {
            let f = (j + a[i]) % 10;
            let g = (j * a[i]) % 10;
            dp[i][f] += dp[i - 1][j];
            dp[i][f] %= MOD;
            dp[i][g] += dp[i - 1][j];
            dp[i][g] %= MOD;
        }
    }
    for i in 0..10 {
        println!("{}", dp[n - 1][i] % MOD);
    }
}
