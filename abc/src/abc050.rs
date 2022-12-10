use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
const MOD: i64 = 1_000_000_007;

#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    }
    let mut b = vec![0; n];
    for i in 0..n {
        b[i] = ((n - 1 - i) as i64 - i as i64).abs();
    }
    a.sort();
    b.sort();

    let ans = if a == b {
        let mut tmp = 1;
        for _ in 0..n / 2 {
            tmp = (tmp * 2) % MOD;
        }
        tmp
    } else {
        0
    };
    println!("{}", ans);
}
