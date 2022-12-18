use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    s:[Chars;n]
        }
    let mut ans = vec![];

    for i in 0..n {
        let si = s[i].clone();
        'inner: for j in i + 1..n {
            let sj = s[j].clone();
            for k in 0..m {
                if si[k] == 'x' && sj[k] == 'x' {
                    continue 'inner;
                }
                if k == m - 1 {
                    let ij = if i > j { (j, i) } else { (i, j) };

                    ans.push(ij);
                }
            }
        }
    }
    ans.sort();
    ans.dedup();
    println!("{}", ans.len());
}
