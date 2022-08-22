use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::ops::{Add, Sub};

pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
    p:u64,
    q:u64,r:u64,
    a:[u64;n]
    }
    let cs = a.cumsum();
    for i in cs.clone() {
        let y = i + p;
        let z = i + p + q;
        let w = i + p + q + r;
        if cs.binary_search(&y).is_ok()
            && cs.binary_search(&z).is_ok()
            && cs.binary_search(&w).is_ok()
        {
            println!("Yes");
            return;
        }
    }
    println!("No");
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

fn c() {
    input! {
    h:usize,
    w:usize,
    g:[Chars;h]
    }
    let mut res = (0, 0);
    let mut visited = HashMap::new();
    loop {
        let x = res.0;
        let y = res.1;
        match g[x][y] {
            'U' if x > 0 => {
                res = (x - 1, y);
            }
            'D' if x < h - 1 => {
                res = (x + 1, y);
            }
            'L' if y > 0 => {
                res = (x, y - 1);
            }
            'R' if y < w - 1 => {
                res = (x, y + 1);
            }
            _ => {
                break;
            }
        }
        if visited.contains_key(&res) {
            println!("-1");
            return;
        }
        visited.insert(res, true);
    }
    println!("{} {}", res.0 + 1, res.1 + 1);
}

fn b() {
    input! {
    n:usize,m:usize,
    mut t:u64,
    a:[u64;n-1],
    c:[(u64,u64);m]
    }
    let mut bonus = HashMap::new();
    for (i, j) in c.iter() {
        bonus.insert(i, j);
    }

    for i in 1..n {
        let q = bonus.get(&(i as u64));
        if q.is_some() {
            t += **q.unwrap();
        }
        if t <= a[i - 1] {
            println!("No");
            return;
        }
        t -= a[i - 1];
    }
    println!("Yes");
}

pub fn a() {
    input! {
    x:u64,
    y:u64,
    n:u64
        }
    if n < 3 {
        println!("{:?}", x * n);
        return;
    }
    if x * 3 > y {
        let mut res = 0;
        let temp = n / 3;
        res += temp * y;
        res += x * (n - temp * 3);
        println!("{:?}", res);
    } else {
        println!("{:?}", x * n);
    }
}
