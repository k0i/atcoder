#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,mut a:[i64;n], mut b:[i64;n]}
    a.sort();
    b.sort();
    let ans = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).abs())
        .sum::<i64>();
    println!("{}", ans);
}
