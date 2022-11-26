use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: f64,
        m: f64,
    }
    let t_n = (n % 12.) / 12. * 360.;
    let m = m / 60. * 360.;
    let n = t_n + m / 12.;
    let ans = (m - n).abs();
    println!("{}", (360. - ans).min(ans));
}
