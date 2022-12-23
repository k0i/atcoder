use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize
        }
    let mut m_div = vec![];
    let mut i = 1;
    while i * i <= m {
        if m % i == 0 {
            m_div.push(i);
            if i != m / i {
                m_div.push(m / i);
            }
        }
        i += 1;
    }
    let mut ans = 0;
    for i in m_div {
        if i * n <= m {
            ans = std::cmp::max(ans, i);
        }
    }
    println!("{}", ans);
}
