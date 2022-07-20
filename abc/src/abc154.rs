use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,k:usize,p:[f64;n]}
    let mut res = 0;
    let mut prob: Vec<f64> = p.iter().map(|x| (x + 1.0) / 2.0).collect();
    let cm = prob.cumsum();
    let mut max = 0.0;
    for i in k..=n {
        let temp = cm[i] - cm[i - k];
        max = f64::max(max, temp);
    }
    println!("{:?}", max);
}

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
