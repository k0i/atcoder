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
