use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::ops::{Div, Mul, Rem};
pub trait Euclidean<T>
where
    T: Copy + Ord + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + Default,
    Self: std::marker::Sized,
{
    fn gcd(x: T, y: T) -> T;
    fn lcm(x: T, y: T) -> Self;
}

impl<T> Euclidean<T> for T
where
    T: Copy + Ord + Mul<Output = Self> + Div<Output = Self> + Rem<Output = Self> + Default,
    Self: std::marker::Sized,
{
    fn gcd(mut x: T, mut y: T) -> T {
        if y == T::default() {
            return x;
        }
        while x != T::default() {
            if x < y {
                std::mem::swap(&mut x, &mut y);
            }
            x = x % y;
        }
        y
    }
    fn lcm(x: T, y: T) -> T {
        let gcd = Self::gcd(x, y);
        x * y / gcd
    }
}

#[fastout]
pub fn main() {
    input! {n:usize,
    a:[i64;n]
    }
    let mut g = 0;
    for i in 0..n - 1 {
        g = Euclidean::gcd(g, (a[i] - a[i + 1]).abs());
    }
    if g == 1 {
        println!("2");
        return;
    }
    println!("1");
}
