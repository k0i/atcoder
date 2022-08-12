use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {a:u128,b:u128,x:u128}

    let mut i = 0u128;
    let mut e = 100000000000000000000u128;
    let mut res = 0;
    loop {
        let mut half = (i + e) / 2;
        if e - i <= 1 {
            res = half;
            break;
        }
        let d = half.to_string().chars().count() as u128;
        if x >= a * half + b * d {
            i = half;
        } else {
            e = half
        }
    }
    if res >= 1000000000 {
        res = 1000000000;
    }
    println!("{:?}", res);
}
