use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
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
