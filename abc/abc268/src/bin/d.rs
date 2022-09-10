use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,m:usize,
    s:[String;n],
    t:[String;m]
        }

    let mut usable = HashSet::new();
    let t_h = HashSet::from_iter(t);
    for k in 1..=16 {
        for i in s.iter().permutations(n) {
            let mut temp = String::new();
            for j in 0..i.len() {
                temp.push_str(i[j]);
                if j != i.len() - 1 {
                    for l in 0..k {
                        temp.push('_');
                    }
                }
            }
            usable.insert(temp);
        }
    }
    let d: Vec<_> = usable.difference(&t_h).collect();
    if d.len() == 0 {
        println!("-1");
        return;
    } else {
        for i in d.iter() {
            if i.len() <= 16 && i.len() >= 3 {
                println!("{}", i);
                return;
            }
        }
        println!("-1");
    }
}
