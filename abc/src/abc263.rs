use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::ops::{Add, Sub};

#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
    l:i64,
    r:i64,
    mut a:[i64;n]
    }
    let (cum, cumr) = cumsum(a, l, r);
    println!("{:?}", cum);
    println!("{:?}", cumr);
}

fn cumsum(s: Vec<i64>, l: i64, r: i64) -> (Vec<i64>, Vec<i64>) {
    let mut sec: Vec<i64> = vec![0; s.len() + 1];
    let mut sec_rev: Vec<i64> = vec![0; s.len() + 1];
    let mut left = l;
    let mut right = r;
    let mut lin = 0;
    let mut rin = 0;
    for i in 0..s.len() {
        sec[i + 1] = sec[i] + s[i];
        sec_rev[i + 1] = sec_rev[i] + s[s.len() - 1 - i];
        if sec[i + 1] >= left && l <= s[i] {
            println!("{}here{:?},{}", i, left, s[i]);
            lin = i;
        }
        if sec_rev[i + 1] >= right && r <= s[i] {
            println!("{}rin{:?},{}", i, left, s[i]);
            rin = s.len() - i;
        }
        left += l;
        right += r;
    }
    println!("{:?}", lin);
    println!("{:?}", rin);
    (sec, sec_rev)
}

fn c() {
    input! {n:usize,m:usize}
    let mut dedup = HashSet::new();
    for mut p in (1..=m).permutations(n) {
        p.sort();
        if dedup.get(&p).is_some() {
            continue;
        }
        dedup.insert(p.clone());
        for c in p {
            print!("{} ", c);
        }
        println!();
    }
}
fn b() {
    input! {
    n:usize,
    p:[usize;n-1]
        }
    let mut res = 1;
    let mut cur = n - 2;
    loop {
        if p[cur] == 1 {
            break;
        }
        cur = p[cur] - 2;
        res += 1;
    }
    println!("{:?}", res);
}

fn a() {
    input! {
    mut a:[usize;5]
        }
    a.sort();
    if a[0] == a[1] && a[1] == a[2] && a[0] == a[2] && a[3] == a[4]
        || a[0] == a[1] && a[2] == a[3] && a[3] == a[4] && a[2] == a[4]
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
