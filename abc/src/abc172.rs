use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {
    n:usize,
    m:usize,
    k:usize,
    a:[usize;n],
    b:[usize;m]
    }
    let mut a_cum = vec![0; n + 1];
    let mut b_cum = vec![0; m + 1];
    for i in 0..n {
        a_cum[i + 1] = a_cum[i] + a[i];
    }
    for i in 0..m {
        b_cum[i + 1] = b_cum[i] + b[i];
    }
    let mut ans = 0;
    let mut b_index = m;
    for i in 0..n + 1 {
        if a_cum[i] > k {
            break;
        }
        while b_index > 0 && a_cum[i] + b_cum[b_index] > k {
            b_index -= 1;
        }
        ans = std::cmp::max(ans, i + b_index);
    }
    println!("{}", ans)
}

fn d() {
    input! {
    n:usize
        }
    let mut ans = 0;
    for i in 1..n + 1 {
        let n = n / i;
        let cnt = (n * (2 * i + (n - 1) * i)) / 2;
        ans += cnt
    }
    println!("{}", ans)
}
