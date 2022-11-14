use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        k: usize,
    }

    if k <= 2 && n > k {
        println!("0");
        return;
    } else if n == 1 {
        println!("{}", k);
        return;
    }

    let m: usize = 1000000007;

    let mut ret = k * (k - 1) % m;
    let mut n = n - 2;

    let mut a = k - 2;
    while n > 0 {
        if n & 1 == 1 {
            ret = (ret * a) % m;
        }
        a = (a * a) % m;
        n >>= 1;
    }

    println!("{}", ret);
}
