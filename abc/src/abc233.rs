use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
        n: usize, k : i64,
        a: [i64; n],
    };

    let mut sa = vec![0_i64; n + 1];
    for i in 0..n {
        sa[i + 1] = sa[i] + a[i];
    }

    let mut ma = HashMap::new();
    let mut res = 0_usize;
    for r in 1..=n {
        ma.entry(sa[r - 1]).and_modify(|e| *e += 1).or_insert(1);
        let key = sa[r] - k;
        if ma.contains_key(&key) {
            res += ma.get(&key).unwrap()
        }
    }
    println!("{}", res);
}

fn c() {
    input! {
    n:usize,
    x:u128,
        }

    let mut num = vec![1];

    for i in 0..n {
        input! {l:usize, ans:[u128;l]}
        let mut temp = vec![];
        for j in num.clone().iter() {
            if *j > x {
                continue;
            }
            for k in ans.iter() {
                temp.push(j * k);
            }
        }
        num = temp;
    }
    println!("{}", num.into_iter().filter(|&r| r == x).count());
}
