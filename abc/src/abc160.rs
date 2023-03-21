use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
    x:i64,
    y:i64
    }
    let mut ans = vec![0; n - 1];
    for i in 1..n {
        for j in i + 1..n + 1 {
            let min = std::cmp::min(
                (j - i) as i64,
                std::cmp::min(
                    (j as i64 - x).abs() + 1 + (y - i as i64).abs(),
                    (j as i64 - y).abs() + 1 + (x - i as i64).abs(),
                ),
            );
            ans[(min - 1) as usize] += 1;
        }
    }
    for a in ans {
        println!("{}", a);
    }
}

fn e() {
    input! {
        mut x:usize,
        mut y:usize,
        a:usize,
        b:usize,
        c:usize,
        mut pa:[u64;a],
        mut qb:[u64;b],
        mut rc:[u64;c]
    }
    let mut ans = 0;
    pa.sort_by_key(|v| Reverse(*v));
    qb.sort_by_key(|v| Reverse(*v));
    pa = pa[..x].to_vec();
    qb = qb[..y].to_vec();

    let mut pqr = [pa, qb, rc].concat();
    pqr.sort_by_key(|v| Reverse(*v));

    for i in 0..x + y {
        ans += pqr[i];
    }

    println!("{}", ans);
}
