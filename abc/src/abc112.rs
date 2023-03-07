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
    input! {n: usize,
    p: [(i64, i64, i64); n]
    }
    for cx in 0..=100 {
        'inner: for cy in 0..=100 {
            let mut h = 0;
            for &(x, y, nh) in &p {
                if nh > 0 {
                    h = nh + (x - cx).abs() + (y - cy).abs();
                    break;
                }
            }

            for &(x, y, nh) in &p {
                if nh > 0 {
                    if h != nh + (x - cx).abs() + (y - cy).abs() {
                        continue 'inner;
                    }
                } else {
                    if h > (x - cx).abs() + (y - cy).abs() {
                        continue 'inner;
                    }
                }
            }

            println!("{} {} {}", cx, cy, h);
            return;
        }
    }
}

fn d() {
    input! {
    n:usize,
    m:usize
        }
    let mut m_div = vec![];
    let mut i = 1;
    while i * i <= m {
        if m % i == 0 {
            m_div.push(i);
            if i != m / i {
                m_div.push(m / i);
            }
        }
        i += 1;
    }
    let mut ans = 0;
    for i in m_div {
        if i * n <= m {
            ans = std::cmp::max(ans, i);
        }
    }
    println!("{}", ans);
}
