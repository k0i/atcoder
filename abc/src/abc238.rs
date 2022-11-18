use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        t: usize,
        ts: [(usize, usize); t],
    };
    for &(a, s) in ts.iter() {
        if a * 2 > s {
            println!("No");
        } else {
            let x = s - (a << 1);
            if x & a == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn c() {
    input! {
        n: u128
    }
    const MOD: u128 = 998244353;
    let mut ans = 0;
    let f = |n: u128| {
        if n % 2 == 0 {
            n / 2 % MOD * (n + 1) % MOD
        } else {
            n % MOD * (n + 1) / 2 % MOD
        }
    };
    for i in 0.. {
        let l = 10u128.pow(i);
        let r = (10u128.pow(i + 1) - 1).min(n);
        ans += f(r - l + 1);
        ans %= 998244353;
        if r == n {
            break;
        }
    }
    println!("{}", ans);
}
