#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut alist: [usize;n],
        mut blist: [usize;n],
        mut clist: [usize;n],
    }

    alist.sort();
    blist.sort();
    clist.sort();

    let mut res = 0;
    for i in blist {
        let mut a = alist.lower_bound(&i);
        let mut c = clist.upper_bound(&i);
        res += a * (n - c);
    }
    println!("{}", res);
}
