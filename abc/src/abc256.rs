#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
pub fn main() {
    d();
}

fn d() {
    input! {n:usize,k:[(u64,u64);n]}
    let mut res = vec![0; 200005];
    for i in 0..n {
        let (l, r) = k[i];
        res[l as usize] += 1;
        res[r as usize] -= 1;
    }
    let imos = CumSum::new(&res);
    let m = imos.sec_ref();
    for i in 1..m.len() {
        if m[i - 1] == 0 && m[i] != 0 {
            print!("{} ", i - 1);
        }
        if m[i - 1] != 0 && m[i] == 0 {
            println!("{}", i - 1);
        }
    }
}

use std::ops::{Add, Sub};

/// CumSum is a struct that contains vec of cumulative sum.
/// ```
/// let x = vec![1, 2, 3];
/// let cs = k0i::cumsum::CumSum::new(&x);
/// assert_eq!(cs.sec_ref(), &[0, 1, 3, 6]);
/// ```
pub struct CumSum<T> {
    sec: Vec<T>,
}

impl<T> CumSum<T>
where
    T: PartialOrd + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    pub fn new(v: &[T]) -> Self {
        let mut sec: Vec<T> = vec![T::default(); v.len() + 1];
        for i in 0..v.len() {
            sec[i + 1] = sec[i] + v[i];
        }
        Self { sec }
    }
    // return reference own sec.
    pub fn sec_ref(&self) -> &Vec<T> {
        &self.sec
    }
    // clone and return own sec.
    pub fn sec(&self) -> Vec<T> {
        self.sec.clone()
    }
    /// return sum of the section that containes [left, right)
    /// ```
    /// let x = vec![1,2,3,4,5];
    /// let cs = k0i::cumsum::CumSum::new(&x);
    /// assert_eq!(cs.sum(2,4),x[2]+x[3]);
    /// assert_eq!(cs.sum(0,4),x.iter().take(4).sum());
    /// ```
    pub fn sum(&self, left: usize, right: usize) -> T {
        self.sec[right] - self.sec[left]
    }
}

fn c() {
    input! {h1:i64,h2:i64,h3:i64,w1:i64,w2:i64,w3:i64}
    let mut res = 0;
    for i in 1..=30 {
        for j in 1..=30 {
            for k in 1..=30 {
                for l in 1..=30 {
                    let first_row = h1 - i - j;
                    if first_row <= 0 {
                        continue;
                    }
                    let second_row = h2 - k - l;
                    if second_row <= 0 {
                        continue;
                    }
                    let first_col = w1 - i - k;
                    if first_col <= 0 {
                        continue;
                    }
                    let second_col = w2 - j - l;
                    if second_col <= 0 {
                        continue;
                    }
                    let last_row = h3 - first_col - second_col;
                    let last_col = w3 - first_row - second_row;
                    if last_row > 0 && last_col > 0 && last_row == last_col {
                        res += 1
                    }
                }
            }
        }
    }
    println!("{}", res);
}
