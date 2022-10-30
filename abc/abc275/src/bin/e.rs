use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

const MOD: usize = 998244353;

fn modpow(base: usize, exp: usize, m: usize) -> usize {
    if exp == 0 {
        1
    } else {
        let half = modpow(base, exp / 2, m);
        let ret = (half * half) % m;
        if exp % 2 == 0 {
            ret
        } else {
            (ret * base) % m
        }
    }
}

fn index(i: usize, n: usize) -> usize {
    if i <= n {
        return i;
    } else {
        return 2 * n - i;
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }
    let mut dp = vec![vec![0usize; n + 1]; k + 1];
    let invm = modpow(m, MOD - 2, MOD);
    dp[0][0] = 1usize;
    let mut ans = 0usize;
    for i in 0..k {
        for j in 0..n {
            for k in 1..=m {
                dp[i + 1][index(j + k, n)] =
                    (dp[i + 1][index(j + k, n)] + (dp[i][j] * invm) % MOD) % MOD;
            }
        }
        ans = (ans + dp[i + 1][n]) % MOD;
        dp[i + 1][n] = 0usize;
    }
    println!("{}", ans);
}
