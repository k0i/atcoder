use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

const MOD: usize = 998244353;

fn power(base: usize, times: usize) -> usize {
    if times == 0 {
        return 1usize;
    }
    if times == 1 {
        return base;
    }
    let temp = power(base, times / 2);
    temp * temp % MOD * power(base, times % 2) % MOD
}

#[fastout]
pub fn main() {
    input! {
    n:usize,
    p:usize
        }

    let mod100 = power(100, MOD - 2);
    let p = p * mod100 % MOD;
    let revp = (MOD + 1 - p) % MOD;
    let mut dp = vec![0; n + 1];
    dp[1] = 1;
    for i in 2..=n {
        dp[i] = (dp[i - 1] * revp + dp[i - 2] * p + 1) % MOD;
    }
    println!("{}", dp[n]);
}

fn a() {
    input! {
        h: usize,
        _: usize,
        s: [String; h]
    }
    println!(
        "{}",
        s.iter()
            .fold(0, |t, s| t + s.chars().filter(|&c| c == '#').count())
    );
}
fn d() {
    input! {
        mut k: usize,
    }
    let mut ans = 1usize;
    let mut p = 2;
    while p * p <= k {
        let mut a = 0;
        while k % p == 0 {
            k /= p;
            a += 1;
        }

        let mut n = 0;
        while a > 0 {
            n += p;
            let mut x = n;
            while x % p == 0 {
                x /= p;
                a -= 1;
            }
        }
        ans = ans.max(n);
        p += 1;
    }
    ans = ans.max(k);
    println!("{}", ans);
}
fn c() {
    input! {
        k: Chars,
        t: Chars,
    };

    for i in 0..k.len() {
        if k[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", k.len() + 1);
}
