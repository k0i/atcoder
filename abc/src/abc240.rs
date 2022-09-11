use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    x:usize,
    ab:[(usize,usize);n],
            }
    let mut dp = vec![vec![0; x + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        let (a, b) = ab[i - 1];
        for j in 0..x {
            if dp[i - 1][j] != 0 {
                if j + a < x + 1 {
                    dp[i][j + a] += dp[i - 1][j];
                }
                if j + b < x + 1 {
                    dp[i][j + b] += dp[i - 1][j];
                }
            }
        }
    }
    if dp[n][x] != 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
