use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {n:usize,m:usize,abm:[(usize,usize);m],k:usize,cdk:[(usize,usize);k]}
    let mut ans = 0;
    for i in 0..=1 << k {
        let mut s = HashSet::new();
        for j in 0..k {
            if i & 1 << j != 0 {
                s.insert(cdk[j].0);
            } else {
                s.insert(cdk[j].1);
            }
        }
        let mut cnt = 0;
        for i in 0..m {
            if s.contains(&abm[i].0) && s.contains(&abm[i].1) {
                cnt += 1;
            }
        }
        ans = ans.max(cnt);
    }
    println!("{}", ans);
}

fn d() {
    input! {
    n:u64
        }
    let mut res = 0;
    let mut i = 1;
    while i * i <= n * 2 {
        if (2 * n) % i == 0 {
            let p = (2 * n) / i;
            let q = i;
            if (p + q - 1) % 2 == 0 && (p - q - 1) % 2 == 0 {
                res += 1
            };
        }
        i += 1;
    }
}
