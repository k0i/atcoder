use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::ops::{Add, Sub};

#[fastout]
pub fn main() {
    input! {
    s:Chars,
    k:i64
        }
    let mut cnt: Vec<i64> = vec![0; s.len() + 1];
    for i in 0..s.len() {
        if s[i] == '.' {
            cnt[i + 1] = cnt[i] + 1;
        } else {
            cnt[i + 1] = cnt[i];
        }
    }
    let mut ans = 0;
    let mut r = 0;
    for i in 0..s.len() {
        while r < s.len() && cnt[r + 1] - cnt[i] <= k {
            r += 1;
        }
        ans = std::cmp::max(ans, r - i);
    }
    println!("{:?}", ans);
}

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
