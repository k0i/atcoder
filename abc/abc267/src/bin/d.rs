use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,m:usize,a:[i64;n]
        }
    let min = std::i64::MIN;
    let mut dp = vec![vec![min; m + 1]; n + 1];
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 0..=m {
            if j > i {
                continue;
            }
            if j == 0 {
                dp[i][j] = 0;
                continue;
            }
            dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i - 1][j - 1] + a[i - 1] * j as i64)
        }
    }
    println!("{}", dp[n][m]);
}
