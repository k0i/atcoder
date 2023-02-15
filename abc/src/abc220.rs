use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
const MOD: usize = 998_244_353;

#[fastout]
pub fn main() {
    input! {
        n: usize,
        d: usize,
    }
    let mut pw2 = vec![1; n * 2 + 2];
    for i in 0..n * 2 + 1 {
        pw2[i + 1] = pw2[i] * 2 % MOD;
    }
    let mut res = 0;
    for i in 0..d + 1 {
        let j = d - i;
        if i.max(j) >= n {
            continue;
        }
        let mut c = pw2[n - i.max(j)] - 1;
        c *= pw2[i.max(1) - 1];
        c %= MOD;
        c *= pw2[j.max(1) - 1];
        c %= MOD;
        res += c;
        res %= MOD;
    }
    res *= 2;
    res %= MOD;
    println!("{}", res);
}
fn d() {
    input! {
    n:usize,
    a:[usize;n]
        }
    let modulo = 998244353;
    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..10 {
            let f = (j + a[i]) % 10;
            let g = (j * a[i]) % 10;
            dp[i][f] += dp[i - 1][j];
            dp[i][f] %= modulo;
            dp[i][g] += dp[i - 1][j];
            dp[i][g] %= modulo;
        }
    }
    for i in 0..10 {
        println!("{}", dp[n - 1][i] % modulo);
    }
}
