use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
    }
    let mut h = HashMap::new();
    for i in 0..n {
        let x = a[i];
        if h.contains_key(&x) {
            h.insert(x, h[&x] + 1);
        } else {
            h.insert(x, 1);
        }
    }
    let mut even = HashSet::new();
    for (k, v) in h.iter_mut() {
        if *v == 1 {
            continue;
        }
        if *v % 2 != 0 {
            *v = 1;
        } else {
            even.insert(*k);
        }
    }
    if even.len() % 2 == 0 {
        for i in even {
            h.insert(i, 1);
        }
    } else {
        let last = even.iter().last().unwrap();
        for i in 0..even.len() - 1 {
            h.insert(*even.iter().nth(i).unwrap(), 1);
        }
        h.insert(*last, 1);
        for (k, v) in h.iter_mut() {
            if *v == 1 {
                *v = 0;
                break;
            }
        }
    }
    println!("{:?}", h.values().sum::<usize>());
}
fn c() {
    input! {mut n: usize};
    let mut res = 0;
    res += n / 11 * 2;
    n %= 11;
    res += n / 6;
    n %= 6;
    if n > 0 {
        res += 1;
    }
    println!("{}", res);
}
