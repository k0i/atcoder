use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,l:usize}
    let mut dp = vec![0; n + l];
    dp[0] = 1;
    let m = 1000000000 + 7;
    for i in 0..n {
        dp[i + l] += dp[i] % m;
        dp[i + 1] += dp[i] % m;
    }
    println!("{:?}", dp[n] % m);
}
