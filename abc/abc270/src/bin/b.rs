use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        x:i64,
        y:i64,
        z:i64
    }
    let mut res = 0;
    if x > 0 {
        if y > x || y < 0 {
            println!("{}", x);
            return;
        }
        if y > 0 && y < z {
            println!("-1");
            return;
        }
        if z < 0 {
            res += z.abs() * 2;
        }
        res += x;
        println!("{}", res);
    } else {
        if y < x || y > 0 {
            println!("{}", x.abs());
            return;
        }
        if y < 0 && y > z {
            println!("-1");
            return;
        }
        if z > 0 {
            res += z * 2;
        }
        res += x.abs();
        println!("{}", res);
        return;
    }
}
