use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
        }
    let mut hs = HashMap::new();
    for i in 0..n {
        hs.entry(a[i]).or_insert(0);
        *hs.get_mut(&a[i]).unwrap() += 1;
    }
    let mut ans = 0;
    'outer: for i in 0..n {
        let cur = a[i];
        if hs[&cur] > 1 {
            continue;
        }
        let mut div = vec![];
        let mut j = 1;
        while j * j <= cur {
            if cur % j == 0 {
                div.push(j);
                if j * j != cur {
                    div.push(cur / j);
                }
            }
            j += 1;
        }
        let v = hs.remove(&cur).unwrap();
        for d in div {
            if hs.get(&d).is_some() {
                hs.insert(cur, v);
                continue 'outer;
            }
        }
        ans += 1;
        hs.insert(cur, v);
    }
    println!("{}", ans);
}
