use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    mut a:[usize;n],
    mut b:[usize;m],
        }
    a.sort();
    b.sort();
    let mut h = vec![false; n];
    for i in b {
        match a.lower_bound(i) {
            Ok(x) => {
                let mut j = x;
                while h[j] {
                    j += 1;
                    if j == n {
                        println!("NO");
                        return;
                    }
                }
                h[j] = true;
            }
            Err(x) => {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
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
