use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        x: isize,
        _a: isize,
        _d: isize,
        n: isize,
    }

    if _d == 0 {
        println!("{}", (x - _a).abs());
        return;
    }

    let a = if _d >= 0 { _a } else { _a + (_d * (n - 1)) };
    let d = _d.abs();

    let last = a + (d * (n - 1));
    if x < a {
        println!("{}", a - x);
    } else if x > last {
        println!("{}", x - last);
    } else {
        let m = (x - a).rem_euclid(d);
        println!("{}", m.min(d - m));
    }
}
