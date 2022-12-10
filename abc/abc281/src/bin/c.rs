use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut t:usize,
    a:[usize;n]
        }

    let sum = a.iter().sum::<usize>();
    if sum < t {
        t %= sum;
    }
    for i in 0..n {
        if t <= a[i] {
            println!("{} {}", i + 1, t);
            return;
        }
        t -= a[i];
    }
}
