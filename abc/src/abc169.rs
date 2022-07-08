#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[u128;n],
        }
    let mut ans = 1;
    let mut overflow = false;
    let mut zero = false;
    for i in 0..n {
        ans *= a[i];
        if ans > 10_u128.pow(18) {
            overflow = true;
        }
        if a[i] == 0 {
            zero = true;
        }
    }
    if zero {
        println!("0");
    } else if overflow {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
