use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    c:usize,
    mut abc:[(Usize1,usize,i64);n]
        }
    let mut event = vec![];
    for i in 0..n {
        event.push((abc[i].0, abc[i].2));
        event.push((abc[i].1, -abc[i].2));
    }
    event.sort();
    let mut res = 0;
    let mut temp = 0;
    let mut fee = 0i64;
    for (x, y) in event {
        if temp != x {
            res += std::cmp::min(c as i64, fee) * (x - temp) as i64;
            temp = x;
        }
        fee += y;
    }
    println!("{}", res);
}
