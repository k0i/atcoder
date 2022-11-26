use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    t:[usize;n]
        }
    let mut l = 1;
    for i in 0..n {
        l = lcm(t[i], l);
    }
    println!("{}", l);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(a: usize, b: usize) -> usize {
    let g = gcd(a, b);
    (a / g) * b
}
