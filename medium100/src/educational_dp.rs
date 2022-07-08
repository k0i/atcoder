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
    input! {
    n:usize,
    r:[(u32,u32,u32);n],
    }
}

fn b() {
    input! {
       n:usize,
       k:usize,
       h:[i64;n]
    }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                dp[i + j] = std::cmp::min(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }
    println!("{}", dp[n - 1]);
}

fn a() {
    input! {
       n:usize,
       h:[i64;n]
    }
    let mut dp = vec![0; n];
    dp[1] = (h[1] - h[0]).abs();
    if n == 2 {
        println!("{:?}", dp[1]);
    } else {
        for i in 2..n {
            dp[i] = std::cmp::min(
                dp[i - 2] + (h[i] - h[i - 2]).abs(),
                dp[i - 1] + (h[i] - h[i - 1]).abs(),
            );
        }
        println!("{:?}", dp[n - 1]);
    }
}
