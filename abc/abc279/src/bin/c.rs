use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    s:[Chars;h],
    t:[Chars;h]
        }

    let mut ht = vec![vec![]; w];
    let mut tt = vec![vec![]; w];
    let mut hs = HashMap::new();
    let mut ts = HashMap::new();
    for i in 0..h {
        for j in 0..w {
            ht[j].push(s[i][j]);
        }
    }
    for i in 0..h {
        for j in 0..w {
            tt[j].push(t[i][j]);
        }
    }
    for i in tt {
        let v = ts.entry(i).or_insert(0);
        *v += 1;
    }

    for i in ht {
        let v = hs.entry(i).or_insert(0);
        *v += 1;
    }
    let mut ans = "Yes";
    for (k, v) in hs.iter() {
        if ts.contains_key(k) {
            if ts[k] != *v {
                ans = "No";
                break;
            }
            ts.remove(k);
            continue;
        } else {
            ans = "No";
            break;
        }
    }
    if ts.len() != 0 {
        ans = "No";
    }
    println!("{}", ans);
}
