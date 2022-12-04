use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        k: Chars,
        t: Chars,
    };

    for i in 0..k.len() {
        if k[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", k.len() + 1);
}
