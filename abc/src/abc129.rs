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
    a:[usize;m]
        }
    let mut dp = vec![0; n + 10];
    let mut broken = vec![false; n + 10];
    let mo = 1000000007;
    for i in a {
        broken[i] = true;
    }
    dp[0] = 1;
    for i in 0..n {
        for j in 1..=2 {
            if !broken[i + j] {
                dp[i + j] += dp[i] % mo;
            }
        }
    }
    println!("{:?}", dp);
    println!("{:?}", dp[n] % mo);
}
