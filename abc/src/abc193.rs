use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::HashSet;
#[fastout]
pub fn main() {
    input! {
        k: usize,
        s: [Bytes; 2],
    }
    let n = k * 9 - 8;
    let mut h = vec![vec![0; 10]; 2];
    let mut c = vec![k; 10];
    for i in 0..2 {
        for j in 0..4 {
            let x = (s[i][j] - b'0') as usize;
            h[i][x] += 1;
            c[x] -= 1;
        }
    }
    let mut res = 0;
    for i in 1..10 {
        if c[i] == 0 {
            continue;
        }
        h[0][i] += 1;
        let p1 = c[i];
        c[i] -= 1;
        for j in 1..10 {
            if c[j] == 0 {
                continue;
            }
            h[1][j] += 1;
            let p2 = c[j];
            let mut sc = vec![0; 2];
            for i in 0..2 {
                for j in 1..10 {
                    sc[i] += j * 10_usize.pow(h[i][j] as u32);
                }
            }
            if sc[0] > sc[1] {
                res += p1 * p2;
            }
            h[1][j] -= 1;
        }
        h[0][i] -= 1;
        c[i] += 1;
    }
    println!("{}", (res as f64) / (n as f64) / ((n - 1) as f64));
}

fn c() {
    input! {n:u64}
    let mut a: u64 = 2;
    let mut b: u64 = 2;
    let mut res = n;
    let mut decre = HashSet::new();
    while a.pow(2) <= n {
        while a.pow(b as u32) <= n {
            if decre.contains(&a.pow(b as u32)) {
                b += 1;
                continue;
            };
            decre.insert(a.pow(b as u32));
            res -= 1;
            b += 1;
        }
        a += 1;
        b = 2;
    }
    println!("{:?}", res);
}
