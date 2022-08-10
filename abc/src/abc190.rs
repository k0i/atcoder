use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:u64
        }
    let mut res = 0;
    let mut i = 1;
    while i * i <= n * 2 {
        if (2 * n) % i == 0 {
            let p = (2 * n) / i;
            let q = i;
            if (p + q - 1) % 2 == 0 && (p - q - 1) % 2 == 0 {
                res += 1
            };
        }
        i += 1;
    }
    println!("{:?}", res * 2);
}
