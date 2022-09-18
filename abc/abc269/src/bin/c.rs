use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let b = format!("{:b}", n);
    let c = b.as_str().chars().collect_vec();
    let mut one = 0;
    for i in 0..c.len() {
        if c[i] == '1' {
            one += 1;
        }
    }
    let mut cur = vec![String::from("0"); c.len()];
    let mut pos = vec![];
    let mut already = HashSet::new();
    println!("0");
    already.insert(cur.clone());
    for i in (0..c.len()).rev() {
        if c[i] == '1' {
            pos.push(i);
            for j in 0..1 << pos.len() {
                for k in 0..pos.len() {
                    if j >> k & 1 == 1 {
                        cur[pos[k]] = String::from("1");
                    } else {
                        cur[pos[k]] = String::from("0");
                    }
                }
                if already.contains(&cur) {
                    continue;
                }
                already.insert(cur.clone());
                println!("{}", i64::from_str_radix(cur.concat().as_str(), 2).unwrap());
            }
        }
    }
}
