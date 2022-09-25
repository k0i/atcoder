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
    k:usize
        }
    let mut res = 0;
    let MOD = 998244353;
    let mut dp = vec![vec![0; 3010]; 51];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=k {
            for l in 1..=m {
                dp[i + 1][j + l] += dp[i][j] % MOD;
                dp[i + 1][j + l] %= MOD;
            }
        }
    }
    for i in n..=k {
        res += dp[n][i] % MOD;
        res %= MOD;
    }
    println!("{}", res % MOD);
}
