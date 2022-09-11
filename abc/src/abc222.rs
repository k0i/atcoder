use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,a:[usize;n],b:[usize;n]
            }
    let max = 3010;
    let mo = 998244353;
    let mut dp = vec![vec![0; max]; max];
    dp[0][0] = 1;
    let mut cumsum = vec![0; max];
    for i in 0..n {
        cumsum[0] = dp[i][0];
        for last in 1..3001 {
            cumsum[last] = cumsum[last - 1] % mo + dp[i][last] % mo;
        }
        for j in a[i]..=b[i] {
            dp[i + 1][j] += cumsum[j];
        }
    }
    let mut res = 0;
    for i in 0..3001 {
        res += dp[n][i] % mo;
        res %= mo;
    }
    println!("{}", res);
}
