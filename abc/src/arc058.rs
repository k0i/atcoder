#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
use superslice::*;
#[fastout]
pub fn main() {
    ip! {
    mut n :usize,
    k:usize,
     d:[usize;k]
        }

    let mut st = HashSet::new();

    for i in 0..k {
        st.insert(d[i]);
    }
    loop {
        let mut str_n = n.to_string().chars().collect::<Vec<char>>();
        let mut flag = false;
        for i in 0..str_n.len() {
            if st.contains(&(str_n[i] as usize - 48)) {
                flag = true;
                break;
            }
        }
        if !flag {
            println!("{}", n);
            return;
        }
        n += 1;
    }
}
