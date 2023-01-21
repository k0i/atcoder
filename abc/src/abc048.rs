use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    x:usize,
    mut a:[usize;n]
    }
    let mut ans = 0;

    for i in 0..n - 1 {
        if x < a[i] {
            ans += a[i] - x;
            a[i] = x;
        }
        if x < a[i] + a[i + 1] {
            ans += a[i] + a[i + 1] - x;
            a[i + 1] = x - a[i];
        }
    }
    println!("{}", ans);
}
fn b() {
    input! {a:usize,b:usize,x:usize}
    if a == 0 {
        println!("{}", div(b, x) + 1);
    } else {
        println!("{}", div(b, x) - div(a - 1, x));
    }
}

fn div(n: usize, x: usize) -> usize {
    n / x
}
