use std::cmp::Ordering;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    q:usize,
    }
    let mut h = std::collections::BinaryHeap::new();
    let mut sum = 0;

    for _ in 0..q {
        input! {
        n:usize
        }
        match n {
            1 => {
                input! {
                mut x:i64
                }
                x -= sum;
                x = -x;
                h.push(x);
            }
            2 => {
                input! {
                mut x:i64
                }
                sum += x;
            }
            _ => {
                let mut x = h.pop().unwrap();
                x = -x;
                x += sum;
                println!("{}", x);
            }
        }
    }
}
fn c() {
    input! {n:usize,m:usize,mut a:[i64;n],mut b:[i64;m]}
    a.sort_unstable();
    b.sort_unstable();
    let mut min = std::i64::MAX;
    for i in 0..n {
        let index = b.lower_bound(&a[i]);
        min = std::cmp::min((b[index] - a[i]).abs(), min);
        if index >= 1 {
            min = std::cmp::min((b[index - 1] - a[i]).abs(), min);
        }
    }
    println!("{:?}", min);
}

// Returns an iterator to specified bound that pointing to the first element in the range.
pub trait Bound<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

impl<T: Ord> Bound<T> for [T] {
    /// Returns an iterator pointing to the first element in the range [first,last) which does not compare less than val.
    ///```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(&4), 2);
    ///```
    /// if the candidate is larger than set's larget item,returns set's last index
    ///```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(&1000), 3);
    ///```
    fn lower_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Less => {
                    low = mid + 1;
                }
                _ => {
                    high = mid;
                }
            }
        }
        // Returns last item's index if x is larger than self's largest item.
        if low == self.len() {
            low - 1
        } else {
            low
        }
    }
    /// Returns an iterator pointing to the first element in the range [first,last) which compares greater than val.
    ///```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(&4), 3);
    ///```
    /// if the candidate is larger than set's larget item,returns set's last index
    ///```
    /// use k0i::bounds::Bound;
    /// let vec = vec![2, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(&1000), 3);
    ///```
    fn upper_bound(&self, x: &T) -> usize {
        let mut low = 0;
        let mut high = self.len();

        while low < high {
            let mid = (low + high) / 2;
            match self[mid].cmp(x) {
                Ordering::Greater => {
                    high = mid;
                }
                _ => {
                    low = mid + 1;
                }
            }
        }
        // Returns last item's index if x is larger than self's smallest item.
        if low == self.len() {
            low - 1
        } else {
            low
        }
    }
}
