use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        wv: [(i64, i64); n],
        x: [i64; m],
        lr: [(Usize1, usize); q],
    }
    for &(l, r) in &lr {
        let mut ev = vec![];
        for i in (0..l).chain(r..m) {
            ev.push((x[i], 1, 0));
        }
        for (i, &(w, v)) in wv.iter().enumerate() {
            ev.push((w, 0, v))
        }
        ev.sort();
        let mut res = 0;
        let mut pq = BinaryHeap::new();
        for &(w, t, v) in &ev {
            if t == 0 {
                pq.push(v);
            } else if let Some(v) = pq.pop() {
                res += v;
            }
        }
        println!("{}", res);
    }
}
fn b() {
    input! {
    a:i64,
    b:i64,
    mut w:i64
        }
    w *= 1000;
    let l = 10_i64.pow(6) + 7;
    let mut min = std::i64::MAX;
    let mut max = -min;
    for i in 0..l {
        if i * a <= w && w <= b * i {
            min = std::cmp::min(min, i);
            max = std::cmp::max(max, i);
        }
    }
    if min == std::i64::MAX {
        println!("UNSATISFIABLE");
    } else {
        println!("{} {}", min, max);
    }
}
