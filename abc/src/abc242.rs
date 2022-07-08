#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {n:usize}
    let mut dp = vec![vec![1; 9]; n];
    for i in 1..n {
        for j in 0..9 {
            match j {
                0 => dp[i][j] = dp[i - 1][j] + dp[i - 1][j + 1],
                8 => dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1],
                _ => dp[i][j] = dp[i - 1][j] + dp[i - 1][j + 1] + dp[i - 1][j - 1],
            }
            dp[i][j] %= 998244353;
        }
    }
    println!("{:?}", dp.last().unwrap().iter().sum::<usize>() % 998244353);
}
