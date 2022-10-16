use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::cmp::Ordering;

// Returns an iterator to specified bound that pointing to the first element in the range.
pub trait Bound<T> {
    fn lower_bound(&self, x: T) -> usize;
    fn upper_bound(&self, x: T) -> usize;
}

impl<T: Ord> Bound<T> for [T] {
    /// Returns an iterator pointing to the first element in the range [first,last) which does not compare less than val.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.lower_bound(3), 2);
    /// assert_eq!(vec.lower_bound(0), 0);
    /// assert_eq!(vec.lower_bound(7), 4);
    /// assert_eq!(vec.lower_bound(2), 1);
    /// ```
    fn lower_bound(&self, x: T) -> usize {
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
        low
    }
    /// Returns an iterator pointing to the first element in the range [first,last) which compares greater than val.
    /// ```
    /// use k0i::bounds::Bound;
    /// let vec = vec![1, 2, 4, 6];
    /// assert_eq!(vec.upper_bound(4), 3);
    /// assert_eq!(vec.upper_bound(0), 0);
    /// assert_eq!(vec.upper_bound(3), 2);
    /// assert_eq!(vec.upper_bound(6), 4);
    /// assert_eq!(vec.upper_bound(7), 4);
    /// ```
    fn upper_bound(&self, x: T) -> usize {
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
        low
    }
}
#[fastout]
pub fn main() {
    input! {
    h:usize,w:usize,
    mut st:(usize,usize),
    n:usize
        }
    st.0 -= 1;
    st.1 -= 1;
    let mut wall_x = HashMap::new();
    let mut wall_y = HashMap::new();
    for i in 0..n {
        input! {
        r:(usize,usize)
        }
        wall_x.entry(r.0 - 1).or_insert(Vec::new()).push(r.1 - 1);
        wall_y.entry(r.1 - 1).or_insert(Vec::new()).push(r.0 - 1);
    }
    for (_, v) in wall_x.iter_mut() {
        v.sort();
    }
    for (_, v) in wall_y.iter_mut() {
        v.sort();
    }
    input! {
    query:usize
        }
    for i in 0..query {
        input! {
        r:(char,usize)
        }
        let lrdu = r.0;
        let step = r.1;
        if lrdu == 'L' {
            let wall = wall_x.get(&st.0).unwrap_or(&vec![]).lower_bound(st.1);
            if wall == 0 {
                if st.1 >= step {
                    st.1 -= step;
                } else {
                    st.1 = 0;
                }
            } else {
                let wall = wall_x.get(&st.0).unwrap()[wall - 1];
                if wall + step < st.1 {
                    st.1 -= step;
                } else {
                    st.1 = wall + 1;
                }
            }
        } else if lrdu == 'R' {
            let wall = wall_x.get(&st.0).unwrap_or(&vec![]).upper_bound(st.1);
            if wall == wall_x.get(&st.0).unwrap_or(&vec![]).len() {
                if st.1 + step < w {
                    st.1 += step;
                } else {
                    st.1 = w - 1;
                }
            } else {
                let wall = wall_x.get(&st.0).unwrap()[wall];
                if wall > st.1 + step {
                    st.1 += step;
                } else {
                    st.1 = wall - 1;
                }
            }
        } else if lrdu == 'U' {
            let wall = wall_y.get(&st.1).unwrap_or(&vec![]).lower_bound(st.0);
            if wall == 0 {
                if st.0 >= step {
                    st.0 -= step;
                } else {
                    st.0 = 0;
                }
            } else {
                let wall = wall_y.get(&st.1).unwrap()[wall - 1];
                if wall + step < st.0 {
                    st.0 -= step;
                } else {
                    st.0 = wall + 1;
                }
            }
        } else {
            let wall = wall_y.get(&st.1).unwrap_or(&vec![]).upper_bound(st.0);
            if wall == wall_y.get(&st.1).unwrap_or(&vec![]).len() {
                if st.0 + step < h {
                    st.0 += step;
                } else {
                    st.0 = h - 1;
                }
            } else {
                let wall = wall_y.get(&st.1).unwrap()[wall];
                if wall > st.0 + step {
                    st.0 += step;
                } else {
                    st.0 = wall - 1;
                }
            }
        }

        println!("{} {}", st.0 + 1, st.1 + 1);
    }
}
