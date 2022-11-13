use std::collections::HashMap;

use proconio::{fastout, input};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
        }
    let mut ab = vec![];
    for i in 0..h {
        input! {c:[usize;w]}
        ab.push(c);
    }
    let mut comb = vec![];
    for i in 1..1 << h {
        let mut temp = vec![];
        for j in 0..h {
            if i >> j & 1 == 1 {
                temp.push(j);
            }
        }
        comb.push(temp);
    }
    let mut ans = 0;
    for c in comb {
        let mut h = HashMap::new();
        'row: for i in 0..w {
            let key = ab[c[0]][i];
            let mut temp = 0;
            for j in &c {
                if ab[*j][i] != key {
                    continue 'row;
                } else {
                    temp += 1;
                }
            }
            let v = h.entry(ab[c[0]][i]).or_insert(0);
            *v += temp;
        }
        ans = std::cmp::max(ans, *h.values().max().unwrap_or(&0));
    }
    println!("{}", ans);
}
