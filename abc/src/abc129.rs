use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    s:[Chars;h]
        }
    let mut vertical = vec![vec![-1]; w];
    let mut horizon = vec![vec![-1]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                horizon[i].push(j as i64);
                vertical[j].push(i as i64);
            }
        }
    }
    for i in 0..h {
        horizon[i].push(w as i64);
    }
    for i in 0..w {
        vertical[i].push(h as i64);
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            let v_end = vertical[j].lower_bound((i as i64)).unwrap();
            let v_start = v_end - 1;
            let h_end = horizon[i].lower_bound((j as i64)).unwrap();
            let h_start = h_end - 1;
            let mut temp = vertical[j][v_end] - vertical[j][v_start] - 1 + horizon[i][h_end]
                - horizon[i][h_start]
                - 1;
            temp -= 1;
            if temp < 0 {
                temp = 0;
            }
            ans = std::cmp::max(ans, temp);
        }
    }
    println!("{}", ans);
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
    a:[usize;m]
        }
    let mut dp = vec![0; n + 10];
    let mut broken = vec![false; n + 10];
    let mo = 1000000007;
    for i in a {
        broken[i] = true;
    }
    dp[0] = 1;
    for i in 0..n {
        for j in 1..=2 {
            if !broken[i + j] {
                dp[i + j] += dp[i] % mo;
            }
        }
    }
    println!("{:?}", dp);
    println!("{:?}", dp[n] % mo);
}
