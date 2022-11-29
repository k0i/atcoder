use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn main() {
    input! {
        N:usize,
        M:usize,
        S:Chars,
        T:Chars
    }
    let x = gcd(N, M);
    for i in 0..x {
        if S[i * (N / x)] != T[i * (M / x)] {
            println!("-1");
            return;
        }
    }
    println!("{}", lcm(N, M));
}
