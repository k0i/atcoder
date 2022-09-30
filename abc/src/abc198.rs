use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    r:f64,
    x:f64,y:f64
        }
    let dis = (x * x + y * y).sqrt();
    let ans = (dis / r).ceil() as i64;
    if ans == 1 && dis != r {
        println!("2");
    } else {
        println!("{}", ans);
    }
}
