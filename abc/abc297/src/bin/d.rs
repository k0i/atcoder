#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
    mut a:usize,
    mut b:usize,
        }
    if a == b {
        println!("0");
        return;
    }
    if a < b {
        let tmp = a;
        a = b;
        b = tmp;
    }
    let mut ans = 0;
    while a != b {
        let cnt = a / b;
        ans += cnt;
        a -= cnt * b;
        if a < b {
            let tmp = a;
            a = b;
            b = tmp;
        }
        if a == 0 || b == 0 {
            break;
        }
    }
    println!("{}", ans - 1);
}
