use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:i64
    }
    let mut m = 998244353;
    if n % m == 0 {
        println!("0");
        return;
    }
    if n > 0 {
        if n <= m {
            println!("{}", n + m);
            return;
        }
        println!("{}", n % m);
    } else {
        m *= -1;
        if n >= m {
            println!("{}", n - m);
            return;
        }
        println!("{}", -(m - n % m));
    }
}
