use std::f64::consts::PI;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,x0:f64,y0:f64,xn2:f64,yn2:f64}
    let center = (
        (xn2 + x0) as f64 / 2.0 as f64,
        (yn2 + y0) as f64 / 2.0 as f64,
    );
    let radian = (2.0 * PI) / n as f64;
    let x_a = radian.cos() * (x0 - center.0) - radian.sin() * (y0 - center.1);
    let y_b = radian.sin() * (x0 - center.0) + radian.cos() * (y0 - center.1);
    println!("{} {}", x_a + center.0, y_b + center.1);
}
fn c() {
    input! {
    n:usize,
    a:[usize;n],
        }
    if n == 1 {
        println!("{}", a[0]);
        return;
    }
    let mut ans = std::usize::MAX;

    for i in 0..1 << n - 1 {
        let mut cur = a[0];
        let mut th = 0;
        for j in 1..n {
            if i & 1 << j - 1 != 0 {
                th ^= cur;
                cur = 0;
            }
            cur |= a[j];
        }
        th ^= cur;
        ans = std::cmp::min(ans, th);
    }
    println!("{}", ans);
}
