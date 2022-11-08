use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    k:usize
        }
    let m = 10usize.pow(9) + 7;
    if k % 9 != 0 {
        println!("0");
        return;
    }
    let mut dp = vec![0; k + 1];
    dp[0] = 1;
    for i in 1..k + 1 {
        for d in 1..10 {
            if i >= d {
                dp[i] += dp[i - d]
            }
        }
        dp[i] %= m
    }

    print!("{}", dp[k] % m);
}
