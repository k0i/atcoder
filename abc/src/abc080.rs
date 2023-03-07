#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        f: [[usize; 10]; n],
        p: [[i64; 11]; n]
    };
    let inf = 1 << 60;
    let mut ans = -inf;
    for bits in 1..1 << 10 {
        let js = (0..10).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        let mut sum = 0_i64;
        for i in 0..n {
            let mut count = 0;
            for j in 0..10 {
                if f[i][j] == 1 && js[j] {
                    count += 1;
                }
            }
            sum += p[i][count];
        }
        ans = ans.max(sum);
    }
    println!("{}", ans);
}
