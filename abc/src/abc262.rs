use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
const M: i64 = 998244353;
#[fastout]
pub fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut ans: i64 = 0;
    for i in 1..=n {
        let mut dp = vec![vec![0; i]; i + 1];
        dp[0][0] = 1;
        for &ai in &a {
            let old_dp = dp.clone();
            for j in 0..i {
                for k in 0..i {
                    let new_k = (ai + k) % i;
                    dp[j + 1][new_k] += old_dp[j][k];
                    dp[j + 1][new_k] %= M;
                }
            }
        }
        ans += dp[i][0];
        ans %= M;
    }

    println!("{}", ans);
}
fn c() {
    input! {
    n:usize,
    mut a:[usize;n]
        }
    let mut temp = 0u64;

    for i in 0..n {
        a[i] -= 1;
        if a[i] == i {
            temp += 1;
        }
    }
    let mut res = temp * (temp - 1) / 2;
    for i in 0..n {
        if a[i] > i && a[a[i]] == i {
            res += 1;
        }
    }
    println!("{}", res);
}
