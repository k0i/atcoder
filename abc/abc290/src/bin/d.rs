#![allow(unused_imports)]
use core::time;
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    collections::{BTreeSet, HashMap, HashSet},
    thread,
};
#[fastout]
pub fn main() {
    input! {t: usize, test_vec: [(usize, usize, usize);t]}

    for (n, d, k) in test_vec {
        let r = d % n;
        let gcd = usize::gcd(n, r);
        let q = n / gcd;

        let ans = (((k - 1) % q) * r + (k - 1) / q) % n;
        println!("{}", ans)
    }
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
