use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[allow(dead_code)]
fn update<T: PartialOrd>(value: T, target: &mut T, cond: std::cmp::Ordering) {
    if value.partial_cmp(target) == Some(cond) {
        *target = value;
    }
}
#[allow(unused_macros)]
macro_rules! chmax {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Greater)
    };
}
#[allow(unused_macros)]
macro_rules! chmin {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Less)
    };
}
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize,
    a:[i64;n]
        }
    let mut res = 0;
    let mut right = 0;
    let mut cm = vec![0; n];
    cm[0] = a[0];
    for i in 1..n {
        cm[i] = cm[i - 1] + a[i];
    }
    for i in 0..n {
        let left = i;
        chmax!(right, left);
        while right < n && get(left, right, &cm) < k {
            right += 1;
        }
        res += n - right;
    }
    println!("{}", res);
}

fn get(l: usize, r: usize, a: &[i64]) -> usize {
    let mut res = a[r];
    if l > 0 {
        res -= a[l - 1];
    }
    res as usize
}

fn c() {
    input! {
    w:f64,
    h:f64,
    x:usize,y:usize
    }
    let ans = w * h / 2.0;
    let ans2 = if x * 2 == w as usize && y * 2 == h as usize {
        1
    } else {
        0
    };
    println!("{} {}", ans, ans2);
}
