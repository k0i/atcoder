use std::{collections::HashSet, iter::FromIterator};

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    }
    let mut len = n - 1;
    for i in 0..n {
        len += s[i].len();
    }
    if 16 < len {
        println!("-1");
        return;
    }
    let t = t.into_iter().collect::<HashSet<String>>();
    for s in s.iter().permutations(n) {
        if let Some(x) = solve(n, "".to_string(), 0, &s, 16 - len + 1, &t) {
            println!("{}", x);
            return;
        }
    }
    println!("-1");
}

fn solve(
    n: usize,
    prefix: String,
    i: usize,
    s: &Vec<&String>,
    m: usize,
    t: &HashSet<String>,
) -> Option<String> {
    let mut x = prefix + s[i];
    if i == n - 1 {
        if !t.contains(&x) && x.len() >= 3 {
            return Some(x);
        }
    } else {
        for j in 0..m {
            x += "_";
            let ret = solve(n, x.clone(), i + 1, s, m - j, t);
            if ret.is_some() {
                return ret;
            }
        }
    }
    return None;
}
