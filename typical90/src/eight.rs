#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
static ATCODER: &str = "atcoder";
#[fastout]
pub fn main() {
    input! {n:usize,
    s:Chars
        }
    let mut dp = vec![vec![0; ATCODER.len() + 1]; n + 1];
    let m: u64 = 1000000007;
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..ATCODER.len() + 1 {
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= m;
            if j < ATCODER.len() && ATCODER.chars().nth(j).unwrap() == s[i] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= m;
            }
        }
    }
    let ans = dp[n][ATCODER.len()];
    println!("{}", ans);
}
