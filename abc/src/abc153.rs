use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    n:usize,
    ab:[(usize,usize);n],
        }

    let mut dp = vec![std::i64::MAX - 10000; h + 1];
    dp[0] = 0;
    for i in 0..n {
        let (a, b) = ab[i];
        for x in 0..=h {
            if x + a <= h {
                dp[x + a] = std::cmp::min(dp[x + a], dp[x] + b as i64);
            } else {
                dp[h] = std::cmp::min(dp[h], dp[x] + b as i64);
            }
        }
    }
    println!("{}", dp[h]);
}
