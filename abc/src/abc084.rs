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
     csf:[(i32,i32,i32);n-1]
    }
    for i in 0..n {
        let mut p = i;
        let mut t = 0;
        while p != n - 1 {
            t = std::cmp::max(t, csf[p].1);
            let m = t % csf[p].2;
            t += (csf[p].2 - m) % csf[p].2;
            t += csf[p].0;
            p += 1;
        }
        println!("{}", t);
    }
}

fn d() {
    input! {
    q:usize,
    lr:[(usize,usize);q],
        }
    let max = 100001;
    let mut prime = vec![true; max];
    let mut like = vec![false; max];
    prime[0] = false;
    prime[1] = false;
    let mut i = 0;
    while i * i < max {
        if prime[i] {
            let mut j = 2;
            while i * j < max {
                prime[i * j] = false;
                j += 1;
            }
        }
        i += 1;
    }
    for i in 0..max {
        if !prime[i] {
            continue;
        }
        if prime[(i + 1) / 2] {
            like[i] = true;
        }
    }
    let mut like_cumsum = vec![0; max + 1];
    for i in 0..max {
        like_cumsum[i + 1] = like_cumsum[i] + like[i] as usize;
    }

    for (l, r) in lr {
        println!("{}", like_cumsum[r + 1] - like_cumsum[l]);
    }
}
