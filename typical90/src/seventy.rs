use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    xy:[(i64,i64);n],
        }
    let mut x = vec![];
    let mut y = vec![];

    for (a, b) in xy.iter() {
        x.push(*a);
        y.push(*b);
    }
    x.sort();
    y.sort();
    let x_median = if n % 2 == 0 {
        (x[n / 2 - 1] + x[n / 2]) / 2
    } else {
        x[n / 2]
    };
    let y_median = if n % 2 == 0 {
        (y[n / 2 - 1] + y[n / 2]) / 2
    } else {
        y[n / 2]
    };
    let mut ans = (0, 0);

    for (a, b) in xy {
        ans.0 += (a - x_median).abs();
        ans.1 += (b - y_median).abs();
    }
    println!("{}", ans.0 + ans.1);
}
