#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    n:usize,
    q:usize,
        }

    let mut bx = vec![];
    for i in 0..n {
        bx.push(BTreeSet::new());
    }
    let mut hhm = HashMap::new();

    for i in 0..q {
        ip! {
        x:usize,
        }

        match x {
            1 => {
                ip! {
                ii:usize,
                j:U1,
                }
                bx[j].insert((ii, i));
                hhm.entry(ii).or_insert(BTreeSet::new()).insert(j);
            }
            2 => {
                ip! {i:U1}
                let tg = bx[i].clone();
                for j in tg.iter() {
                    print!("{} ", j.0);
                }
                println!();
            }
            _ => {
                ip! {i:usize}
                let tg = hhm.get(&i).unwrap();
                for j in tg.iter() {
                    print!("{} ", j + 1);
                }
                println!();
            }
        }
    }
}
