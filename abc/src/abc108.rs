use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
pub fn main() {
    input! {N:i64,K:i64}
    let mut ans = 0;
    for a in 1..=N {
        let m1 = a % K;
        let m2 = (K - m1) % K;
        if (m2 + m2) % K != 0 {
            continue;
        }
        let d = (N + 1) / K;
        let m = (N + 1) % K;
        let mut x = d;
        if m2 == 0 {
            x -= 1;
        }
        if m >= m2 + 1 {
            x += 1;
        }
        ans += x * x;
    }
    println!("{}", ans);
}
