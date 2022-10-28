use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::BTreeMap;
use std::ops::Bound::{Included, Unbounded};
#[fastout]
pub fn main() {
    c()
}

fn d() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
    };
    let mut result = vec![-1; n];
    let mut bm = BTreeMap::new();
    for (i, x) in p.into_iter().enumerate() {
        match bm.range(x..).next() {
            None => {
                bm.insert(x, vec![x]);
            }
            Some((&k, _)) => {
                let mut v = bm.remove(&k).unwrap();
                v.push(x);
                bm.insert(x, v);
            }
        };
        if bm.get(&x).unwrap().len() == k {
            bm.remove(&x).map(|vs| {
                for v in vs {
                    result[v - 1] = (i + 1) as isize;
                }
            });
        }
    }
    for i in result {
        println!("{}", i);
    }
}

fn c() {
    input! {n:usize,x:usize,y:usize}
    let mut r = vec![0; n];
    let mut b = vec![0; n];
    b[0] = 1;
    for i in 1..n {
        b[i] = r[i - 1] + b[i - 1] * y;
        r[i] = r[i - 1] + b[i] * x;
    }
    println!("{:?}", r.last().unwrap());
}

// my b() implementation is so ugly. Intended refactoring after reading commentary.
fn b_retry() {
    input! {
    n:usize,
    x:usize,
    y:usize,
    z:usize,
    mut a:[u64;n],
    mut b:[u64;n],
    }
    let mut flag = vec![false; n];
    for i in 0..x {
        let mut po = std::usize::MAX;
        for j in 0..n {
            if !flag[j] {
                if po == std::usize::MAX || a[po] < a[j] {
                    po = j;
                }
            }
        }
        flag[po] = true;
    }
    for i in 0..y {
        let mut po = std::usize::MAX;
        for j in 0..n {
            if !flag[j] {
                if po == std::usize::MAX || b[po] < b[j] {
                    po = j;
                }
            }
        }
        flag[po] = true;
    }
    for i in 0..z {
        let mut po = std::usize::MAX;
        for j in 0..n {
            if !flag[j] {
                if po == std::usize::MAX || a[po] + b[po] < a[j] + b[j] {
                    po = j;
                }
            }
        }
        flag[po] = true;
    }
    for i in 0..n {
        if flag[i] {
            println!("{}", i + 1);
        }
    }
}

fn b() {
    input! {
    n:usize,
    x:usize,
    y:usize,
    z:usize,
    mut a:[u64;n],
    mut b:[u64;n],
    }
    let mut mtup = vec![];
    let mut etup = vec![];
    let mut res = vec![];
    let mut tup = vec![];
    for i in 0..n {
        mtup.push((i + 1, a[i]));
        etup.push((i + 1, b[i]));
        tup.push((i + 1, a[i] + b[i]));
    }
    mtup.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for i in 0..x {
        res.push(mtup[i].0);
    }
    etup.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    let mut en = 0;
    while en < y {
        let r = etup[0];
        etup.remove(0);
        if res.contains(&r.0) {
            continue;
        }
        res.push(r.0);
        en += 1;
    }
    let mut ee = 0;
    tup.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    while ee < z {
        let r = tup[0];
        tup.remove(0);
        if res.contains(&r.0) {
            continue;
        }
        res.push(r.0);
        ee += 1;
    }
    res.sort();
    for i in res {
        println!("{}", i);
    }
}

fn a() {
    input! {
        n:Chars
    }
    if n[1] == n[0] && n[0] == n[2] && n[1] == n[2] {
        println!("-1");
        return;
    }
    if n[0] == n[1] {
        println!("{}", n[2]);
    } else if n[1] == n[2] {
        println!("{}", n[0]);
    } else {
        println!("{}", n[1]);
    }
}
