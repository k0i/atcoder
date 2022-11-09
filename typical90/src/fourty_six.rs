use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[u64;n],
    mut b:[u64;n],
    mut c:[u64;n],
        }
    let mut ans = 0usize;
    let mut hash_a = HashMap::new();
    let mut hash_b = HashMap::new();
    let mut hash_c = HashMap::new();
    for i in 0..n {
        a[i] %= 46;
        b[i] %= 46;
        c[i] %= 46;
        *hash_a.entry(a[i]).or_insert(0) += 1usize;
        *hash_b.entry(b[i]).or_insert(0) += 1usize;
        *hash_c.entry(c[i]).or_insert(0) += 1usize;
    }
    for (ka, va) in hash_a.iter() {
        for (kb, vb) in hash_b.iter() {
            for (kc, vc) in hash_c.iter() {
                if (ka + kb + kc) % 46 == 0 {
                    ans += va * vb * vc;
                }
            }
        }
    }

    println!("{}", ans);
}
