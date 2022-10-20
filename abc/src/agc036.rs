use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        s:u64,
    };
    let m = 1_000_000_000;
    if s % m == 0 {
        let c = 0;
        let d = s / m;
        println!("{} {} {} {} {} {}", 0, 0, m, 1, c, d);
    } else {
        let c = m - s % m;
        let d = s / m + 1;
        println!("{} {} {} {} {} {}", 0, 0, m, 1, c, d);
    }
}
