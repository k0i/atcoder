#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    a:[usize;n],
    q:usize,
    lr:[(usize,usize);q]
        }
    let mut cmsm = vec![];
    cmsm.push(0);
    for i in 0..n {
        if i == 0 {
            continue;
        } else {
            if (i + 1) % 2 == 1 && i > 0 {
                cmsm.push(cmsm[i - 1] + a[i] - a[i - 1]);
            } else {
                cmsm.push(cmsm[i - 1]);
            }
        }
    }
    for i in 0..q {
        let (l, r) = lr[i];
        let l_ind = a.lower_bound(&l);
        let r_ind = a.lower_bound(&r);
        let mut ans = cmsm[r_ind] - cmsm[l_ind];
        if l_ind % 2 == 0 {
            ans += a[l_ind] - l;
        }
        if r_ind % 2 == 0 {
            ans -= a[r_ind] - r;
        }
        println!("{}", ans);
    }
}
