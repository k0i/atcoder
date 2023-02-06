use std::collections::HashSet;

use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    d()
}

fn d() {
    input! {
        w: usize,
    }
    println!("298");
    for i in 1..100 {
        print!("{} ", i);
    }
    for i in 1..100 {
        print!("{} ", i * 100);
    }
    for i in 1..=100 {
        print!("{} ", i * 10000);
    }
}
pub fn c() {
    input! {
    n:usize,
    a:[(String,u64);n]

        }
    let mut max = 0;
    let mut res = 0;
    let mut unique = HashSet::new();
    for i in 0..n {
        if unique.contains(&a[i].0) {
            continue;
        }
        unique.insert(a[i].0.clone());
        if max < a[i].1 {
            max = a[i].1;
            res = (i + 1) as u64
        }
    }
    println!("{}", res);
}
#[fastout]
pub fn b() {
    input! {
    n:usize,
    w:u64,
    a:[u64;n],
    }
    let mut s = Vec::new();
    for i in a {
        if i < w {
            s.push(i);
        }
    }
    let mut global = HashSet::new();
    for i in 1..=3 {
        let e: Vec<_> = s.iter().combinations(i).collect();
        for j in e {
            let c = j.clone().into_iter().sum::<u64>();
            if c <= w {
                global.insert(c);
            }
        }
    }
    println!("{}", global.len());
}

#[fastout]
pub fn a() {
    input! {
    n:Chars
        }
    let mut res: String = String::new();
    let mut i = 0;
    loop {
        res.push(n[i]);
        i += 1;
        if res.len() == 6 {
            break;
        }
        if i == n.len() {
            i = 0;
        }
    }
    println!("{}", res);
}
