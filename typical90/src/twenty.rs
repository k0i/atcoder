#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
     a:u64,
    b:u64,
    c:u64
        }
    let mut c_pow = 1;
    for i in 0..b {
        c_pow *= c;
    }
    match a < c_pow {
        true => println!("{}", "Yes"),
        false => println!("{}", "No"),
    }
}
