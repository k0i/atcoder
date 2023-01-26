use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:usize,
    b:usize,
    x:[usize;n]
        }
    let mut ans = 0;

    for i in 0..n - 1 {
        ans += std::cmp::min((x[i + 1] - x[i]) * a, b);
    }
    println!("{}", ans);
}
fn c() {
    input! {
        n:u64
    }
    let m = 1000000007;
    let mut hm = HashMap::new();
    let mut cnt: u64 = 1;

    for i in 2..=n {
        let mut tmp = i;
        for j in 2..=n {
            while tmp % j == 0 {
                *hm.entry(j).or_insert(1) += 1;
                tmp = tmp / j;
            }
        }
    }

    for (k, v) in hm.iter() {
        cnt *= v;
        cnt %= m;
    }
    println!("{}", cnt);
}
