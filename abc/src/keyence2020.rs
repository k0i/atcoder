use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    b()
}

fn b() {
    input! {
        n: usize,
        xlxl: [(i32, i32); n],
    };
    let mut rs: Vec<_> = xlxl.into_iter().map(|(x, l)| (x - l, x + l)).collect();
    rs.sort_by_key(|lr| lr.1);

    let mut x = std::i32::MIN;
    let mut ans = 0;
    for (l, r) in rs {
        if x <= l {
            ans += 1;
            x = r;
        }
    }
    println!("{}", ans);
}

fn c() {
    input! {
    n:usize,
    k:usize,
    s:usize,
        }
    let mut ans = vec![];
    if s < 1000000000 {
        for i in 0..k {
            ans.push(s);
        }
        for i in 0..n - k {
            ans.push(1000000000);
        }
    } else {
        for i in 0..k {
            ans.push(s);
        }
        for i in 0..n - k {
            ans.push(1);
        }
    }
    for i in 0..n {
        print!("{} ", ans[i]);
    }
}
