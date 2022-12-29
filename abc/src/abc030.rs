use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:usize,
    m:usize,
    x:usize,
    y:usize,
    a:[usize;n],
    b:[usize;m],
       }
    let mut time = 0;
    let mut ans = 0;
    let mut a_to_b = true;
    'outer: loop {
        if a_to_b {
            match a.lower_bound(time) {
                Ok(i) => {
                    time = a[i] + x;
                    a_to_b = false;
                }
                Err(_) => {
                    break 'outer;
                }
            }
        } else {
            match b.lower_bound(time) {
                Ok(i) => {
                    time = b[i] + y;
                    ans += 1;
                    a_to_b = true;
                }
                Err(_) => {
                    break 'outer;
                }
            }
        }
    }
    println!("{}", ans);
}

use std::cmp::Ordering;

pub trait Bound<T> {
    fn lower_bound(&self, x: T) -> Result<usize, usize>;
    fn upper_bound(&self, x: T) -> Result<usize, usize>;
}

impl<T: Ord> Bound<T> for [T] {
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

fn b() {
    input! {
        n: f64,
        m: f64,
    }
    let t_n = (n % 12.) / 12. * 360.;
    let m = m / 60. * 360.;
    let n = t_n + m / 12.;
    let ans = (m - n).abs();
    println!("{}", (360. - ans).min(ans));
}
