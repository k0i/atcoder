use std::collections::BTreeMap;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
fn modulo(x: i64, m: i64) -> i64 {
    ((x % m) + m) % m
}
const MOD: usize = 998244353;

#[fastout]
pub fn main() {
    input! {
      n: usize,
      m: usize,
      k: usize,
    }
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for j in 1..=m {
        dp[1][j] = 1;
    }
    for i in 1..n {
        let mut sum = 0;
        for j in (1..=m).rev() {
            sum += dp[i][j];
        }
        for j in (1..=k).rev() {
            sum -= dp[i][j];
        }
        for j in 1..=m {
            dp[i + 1][j] = sum;
            dp[i + 1][j] %= MOD;
            if j + k <= m {
                sum -= dp[i][j + k];
            }
            if j + 1 >= k && j + 1 - k <= m {
                sum += dp[i][j + 1 - k];
            }
        }
    }
    println!("{}", dp[n].iter().sum::<usize>() % MOD);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[fastout]
fn c() {
    input! {
    n:usize
        }
    let mut h = BTreeMap::new();
    for _ in 0..n {
        input! {a:usize}
        match a {
            1 => {
                input! {x:usize}
                let v = h.entry(x).or_insert(0);
                *v += 1;
            }
            2 => {
                input! {x:usize,y:usize}
                let v = h.entry(x).or_insert(0);
                *v -= std::cmp::min(*v, y);
                if *v == 0 {
                    h.remove(&x);
                }
            }
            _ => {
                println!(
                    "{}",
                    h.iter().last().unwrap().0 - h.iter().next().unwrap().0
                );
            }
        }
    }
}
