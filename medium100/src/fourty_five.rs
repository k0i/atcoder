#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
       mut n:u64,
       mut m:u64
    }
    if m / 2 <= n {
        println!("{}", m / 2);
    } else {
        println!("{}", n + (m - 2 * n) / 4);
    }
}
