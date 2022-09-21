use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let ch = ['c', 'h', 'o', 'k', 'u', 'd', 'a', 'i'];
    let mut dp = vec![vec![0; ch.len() + 1]; s.len() + 1];
    dp[0][0] = 1;
    for i in 0..s.len() {
        for j in 0..=ch.len() {
            if j < ch.len() && s[i] == ch[j] {
                dp[i + 1][j + 1] += dp[i][j];
                dp[i + 1][j + 1] %= 1000000007;
            }
            dp[i + 1][j] += dp[i][j];
            dp[i + 1][j] %= 1000000007;
        }
    }
    println!("{}", dp[s.len()][ch.len()]);
    println!("{:?}", dp);
}
