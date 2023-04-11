#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        q: usize,
    }
    let mut a = vec![];
    let mut cur = None;
    let mut map = BTreeMap::new();
    for _ in 0..q {
        input! { s: String }
        match s.as_str() {
            "ADD" => {
                input! { x: usize }
                let par = cur;
                a.push((par, x));
                cur = Some(a.len() - 1);
            }
            "DELETE" => {
                if let Some(i) = cur {
                    cur = a[i].0;
                }
            }
            "SAVE" => {
                input! { y: usize }
                map.insert(y, cur);
            }
            "LOAD" => {
                input! { z: usize }
                cur = *map.entry(z).or_insert(None);
            }
            _ => unreachable!(),
        }
        if let Some(i) = cur {
            print!("{} ", a[i].1);
        } else {
            print!("-1 ");
        }
    }
    println!();
}
