use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,k:usize
        }
    let mut res: f64 = 0.0;
    for i in 1..=n {
        let mut prob = 1.0 / n as f64;
        if i >= k {
            res += prob;
            continue;
        }
        let mut j = 0;
        while i * 2usize.pow(j) < k {
            j += 1;
            prob *= 1.0 / 2.0;
        }
        res += prob;
    }
    println!("{}", res);
}
