#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    a:usize,
    b:usize
        }
    let ans = f(Euclidean::gcd(a, b));
    println!("{:?}", ans.len());
}

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
    /// ```
    /// use k0i::euclidean::Euclidean;
    /// assert_eq!(Euclidean::gcd(24, 32),8);
    /// ```
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
    /// Returns LCM (Least Common Dividor) of x and y
    /// ```
    /// use k0i::euclidean::Euclidean;
    /// assert_eq!(Euclidean::lcm(24,36), 72);
    /// ```
    fn lcm(x: T, y: T) -> T {
        let gcd = Self::gcd(x, y);
        x * y / gcd
    }
}

fn f(a: usize) -> Vec<(usize, usize)> {
    let mut a = a;
    let mut v = vec![];

    v.push((1, 1));

    for p in 2.. {
        if a == 1 {
            break;
        }
        let mut n = 0;
        while a % p == 0 {
            n += 1;
            a /= p;
        }
        if n != 0 {
            v.push((p, n));
            continue;
        }
        if a < p * p {
            v.push((p, 1));
            break;
        }
    }

    return v;
}
