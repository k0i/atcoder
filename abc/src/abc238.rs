use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
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
