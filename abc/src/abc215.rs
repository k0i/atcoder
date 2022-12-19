use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut hs = HashSet::new();
    for i in 0..n {
        let mut j = 1;
        while j * j <= a[i] {
            if a[i] % j == 0 {
                hs.insert(j);
                hs.insert(a[i] / j);
            }
            j += 1;
        }
    }
    hs.remove(&1);

    let mut ng = HashSet::new();
    for x in hs.into_iter() {
        let mut i = 1;
        while x * i <= m {
            ng.insert(x * i);
            i += 1;
        }
    }

    let mut res = vec![];
    for i in 1..=m {
        if !ng.contains(&i) {
            res.push(i);
        }
    }
    res.sort();
    println!("{}", res.len());
    for r in res.into_iter() {
        println!("{}", r);
    }
}
