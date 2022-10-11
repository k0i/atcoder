use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[i64;n]
        }
    let mut odd = vec![];
    let mut even = vec![];
    for i in a.clone() {
        if i % 2 == 0 {
            even.push(i);
        } else {
            odd.push(i);
        }
    }
    let mut res = -1;
    even.sort_by(|a, b| b.cmp(a));
    odd.sort_by(|a, b| b.cmp(a));

    if even.len() >= 2 {
        res = std::cmp::max(res, even[0] + even[1]);
    }
    if odd.len() >= 2 {
        res = std::cmp::max(res, odd[0] + odd[1]);
    }
    println!("{}", res);
}
