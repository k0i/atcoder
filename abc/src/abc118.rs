use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[usize;n],
        }
    let mut g = 0;
    for i in a {
        g = gcd(i, g);
    }
    println!("{}", g);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if b > a {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    while a % b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    b
}
