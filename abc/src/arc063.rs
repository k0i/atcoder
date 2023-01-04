use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let len = s.len();
    let mut i = 0;
    let mut vv = vec![];
    let mut cur = s[0];
    while i < len {
        let mut ve = vec![];
        while i < len && s[i] == cur {
            ve.push(s[i]);
            i += 1;
        }
        vv.push(ve);
        if i < len {
            cur = s[i];
        }
    }
    println!("{}", vv.len() - 1);
}
