use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n],
    q:usize,
    rlx:[(Usize1,Usize1,usize);q]
        }
    let mut hm = HashMap::new();
    for i in 0..n {
        hm.entry(a[i]).or_insert(vec![]).push(i);
    }
    for i in hm.values_mut() {
        i.sort();
    }
    for i in 0..q {
        let (l, r, x) = rlx[i];
        if !hm.contains_key(&x) {
            println!("0");
            continue;
        }
        let r_i = match hm[&x].upper_bound(r) {
            Ok(i) => i,
            Err(i) => i,
        };
        let l_i = match hm[&x].lower_bound(l) {
            Ok(i) => i,
            Err(i) => i,
        };
        println!("{}", r_i - l_i);
    }
}
use std::cmp::Ordering;

// Returns an iterator to specified bound that pointing to the first element in the range.
pub trait Bound<T> {
    fn lower_bound(&self, x: T) -> Result<usize, usize>;
    fn upper_bound(&self, x: T) -> Result<usize, usize>;
}

impl<T: Ord> Bound<T> for [T] {
    /// Returns an iterator pointing to the first element in the range [first,last) which does not compare less than val.
    /// If the iterator is empty, the function returns Err(0).
    /// If the given element larger than all elements in the range, the function returns Err(length of the range).
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(0), Ok(0));
    /// assert_eq!(vec.lower_bound(2), Ok(1));
    /// assert_eq!(vec.lower_bound(3), Ok(2));
    /// assert_eq!(vec.lower_bound(0), Ok(0));
    /// assert_eq!(vec.lower_bound(6), Ok(3));
    /// assert_eq!(vec.lower_bound(7), Err(4));
    /// ```
    fn lower_bound(&self, x: T) -> Result<usize, usize> {
        if self.is_empty() {
            return Err(0);
        }
        if self.iter().last().unwrap() < &x {
            return Err(self.len());
        }
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        Ok(low)
    }
    /// Returns an iterator pointing to the first element in the range [first,last) which compares greater than val.
    /// If the iterator is empty, the function returns Err(0).
    /// If the given element larger than all elements in the range, the function returns Err(length of the range).
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(4), Ok(3));
    /// assert_eq!(vec.upper_bound(0), Ok(0));
    /// assert_eq!(vec.upper_bound(3), Ok(2));
    /// assert_eq!(vec.upper_bound(6), Err(4));
    /// assert_eq!(vec.upper_bound(7), Err(4));
    /// ```
    fn upper_bound(&self, x: T) -> Result<usize, usize> {
        if self.is_empty() {
            return Err(0);
        }
        if self.iter().last().unwrap() <= &x {
            return Err(self.len());
        }
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(&x) {
                Ordering::Greater => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
        Ok(low)
    }
}

fn c() {
    input! {
    n:usize,
    m:usize,
    k:usize
        }
    let mut res = 0;
    let modulo = 998244353;
    let mut dp = vec![vec![0; 3010]; 51];
    dp[0][0] = 1;
    for i in 0..n {
        for j in 0..=k {
            for l in 1..=m {
                dp[i + 1][j + l] += dp[i][j] % modulo;
                dp[i + 1][j + l] %= modulo;
            }
        }
    }
    for i in n..=k {
        res += dp[n][i] % modulo;
        res %= modulo;
    }
    println!("{}", res % modulo);
}
