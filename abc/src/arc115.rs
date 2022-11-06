use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        _: usize,
        s: [Chars; n]
    };
    let mut e = 0usize;
    let mut o = 0usize;
    for s_i in s {
        if s_i.iter().copied().filter(|&s_ij| s_ij == '1').count() % 2 == 0 {
            e += 1;
        } else {
            o += 1;
        }
    }
    println!("{}", e * o);
}
