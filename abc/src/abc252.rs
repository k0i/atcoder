use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: i32,
        a: [Chars; n],
    }
    let rd: Vec<Vec<u8>> = a
        .into_iter()
        .map(|a| a.into_iter().map(|aa| aa as u8 - b'0').collect::<Vec<u8>>())
        .collect();
    let mut ans = 10000;
    for i in 0usize..10usize {
        let mut tmp = 0;
        let mut used = vec![false; 10001];
        for r in rd.iter() {
            let mut num = 0;
            for (ii, ri) in r.iter().enumerate() {
                if *ri as usize == i {
                    num = ii;
                    break;
                }
            }
            while used[num] {
                num += 10
            }
            used[num] = true;
            tmp = tmp.max(num);
        }
        ans = ans.min(tmp);
    }
    println!("{}", ans);
}
fn d() {
    input! {n:usize,
    a:[usize;n],
    }
    let nc3 = |n: i64| n * (n - 1) * (n - 2) / 6;
    let nc2 = |n: i64| n * (n - 1) / 2;
    let mut dup = HashMap::new();
    for i in a {
        let v = dup.entry(i).or_insert(0);
        *v += 1;
    }
    let mut res = nc3(n as i64);
    for (k, v) in dup.iter() {
        res = res - nc2(*v as i64) * ((n - v) as i64) - nc3(*v as i64);
    }
    println!("{:?}", res);
}
