use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        mut l: u128,
        r: u128,
    }
    const MOD: u128 = 1_000_000_007;
    let mut ten = 10;
    let mut d = 1;
    while ten <= l {
        ten *= 10;
        d += 1
    }
    let mut ans = 0;
    while l <= r {
        let now = if r < ten {
            r * (r + 1) / 2 - l * (l - 1) / 2
        } else {
            ten * (ten - 1) / 2 - l * (l - 1) / 2
        };
        ans += now * d;
        ans %= MOD;
        l = ten;
        ten *= 10;
        d += 1;
    }
    println!("{}", ans);
}
