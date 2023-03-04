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
        n : usize,
        k : usize,
        s : Chars
    }

    let mut rle = vec![(s[0], 0)];
    for &c in s.iter() {
        if rle.last().unwrap().0 == c {
            rle.last_mut().unwrap().1 += 1;
        } else {
            rle.push((c, 1));
        }
    }

    if s[0] == '0' {
        rle.insert(0, ('1', 0));
    }
    if *s.last().unwrap() == '0' {
        rle.push(('0', 0));
    }

    let mut now = rle.iter().take(2 * k + 1).fold(0, |acc, &(_, v)| acc + v);
    let mut ans = now;

    for i in (2 * k + 1..rle.len()).step_by(2) {
        now -= rle[i - 2 * k - 1].1;
        now -= rle[i - 2 * k].1;
        now += rle[i].1;
        now += rle[i + 1].1;
        ans = ans.max(now);
    }

    println!("{}", ans);
}
