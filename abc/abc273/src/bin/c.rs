use std::collections::BTreeSet;

use im_rc::HashMap;
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
        }
    let mut b = BTreeSet::new();
    for i in a.clone() {
        b.insert(i);
    }
    let c: Vec<_> = b.iter().collect();
    let mut bound = HashMap::new();
    for i in 0..b.len() {
        bound.insert(c[i], c.len() - 1 - i);
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let cur = *bound.get(&a[i]).unwrap();
        ans[cur] += 1;
    }
    for i in ans {
        println!("{}", i);
    }
}
