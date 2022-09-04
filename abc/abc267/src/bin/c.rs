use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

use std::ops::{Add, Sub};

/// Returns the cumulative sum.
/// ```
/// use k0i::cumsum::CumSum;
/// let x = vec![1, 2, 3,7];
/// let cumsum = x.cumsum();
/// assert_eq!(cumsum, [0, 1, 3, 6, 13]);
/// ```

pub trait CumSum<T> {
    fn cumsum(&self) -> Vec<T>;
}
impl<T> CumSum<T> for [T]
where
    T: PartialOrd + Copy + Default + Add<Output = T> + Sub<Output = T>,
{
    fn cumsum(&self) -> Vec<T> {
        let mut sec: Vec<T> = vec![T::default(); self.len() + 1];
        for i in 0..self.len() {
            sec[i + 1] = sec[i] + self[i];
        }
        sec
    }
}
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    a:[i64;n]
        }
    let mut init = 0;
    for i in 0..m {
        init += a[i] * (i + 1) as i64;
    }
    let cm = a.cumsum();
    let mut res = init;
    for i in m..n {
        let cums = cm[i] - cm[i - m];
        init = init - cums + a[i] * (m as i64);
        res = std::cmp::max(res, init);
    }
    println!("{}", res);
}
