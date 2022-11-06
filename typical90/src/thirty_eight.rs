use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {mut a:u128,mut b:u128}
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    let g = gcd(a, b);
    if a * b / g > 10u128.pow(18) {
        println!("Large");
        return;
    }
    println!("{}", a * b / g);
}

fn gcd(x: u128, y: u128) -> u128 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}
