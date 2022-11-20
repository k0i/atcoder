use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut n:usize,
    mut m:usize
        }
    if n < m {
        std::mem::swap(&mut n, &mut m);
    }

    if n - m > 1 {
        println!("0");
        return;
    }

    let mo = 1000000007;

    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
        ans %= mo;
    }
    for i in 1..=m {
        ans *= i;
        ans %= mo;
    }
    if m == n - 1 {
        println!("{}", ans % mo);
        return;
    } else {
        ans *= 2;
        ans %= mo;
        println!("{}", ans % mo);
    }
}
