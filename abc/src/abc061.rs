use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeMap, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
       n:usize,
       k:usize,
    mut   ab:[(usize,usize);n]
           }

    ab.sort_by_key(|x| x.0);
    let mut hm = BTreeMap::new();
    for (a, b) in ab.iter() {
        let v = hm.entry(a).or_insert(0);
        *v += b;
    }
    let mut i = 0;
    for (key, value) in hm.iter() {
        i += value;
        if i >= k {
            println!("{}", key);
            return;
        }
    }
}
