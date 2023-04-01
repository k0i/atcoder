use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    a()
}

fn a() {
    input! {
    n:usize,
    c:usize,
    k:usize,
     mut t:[usize;n]
    }
    let mut ans = 0;
    t.sort();
    let mut start = t[0];
    let mut count = 0;
    let mut i = 0;
    while i < n {
        if t[i] - start <= k && count < c {
            count += 1;
            i += 1;
        } else {
            ans += 1;
            start = t[i];
            count = 0;
        }
    }
    if count > 0 {
        ans += 1;
    }
    println!("{}", ans);
}

fn b() {
    input! {
        n: usize,
        mut a: [i64; n],
    }

    a.sort();

    let mut result = n;
    let mut sum = 0;

    for i in 0..n - 1 {
        sum += a[i];
        if sum * 2 < a[i + 1] {
            result = n - i - 1;
        }
    }
    println!("{}", result);
}
