use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:[Chars;10]
        }
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    let mut d = 0;
    for i in 0..10 {
        for j in 0..s[i].len() {
            if s[i][j] == '#' {
                if a == 0 {
                    a = i + 1
                };
                if c == 0 {
                    c = j + 1
                };
                if b < i + 1 {
                    b = i + 1
                };
                if j + 1 == s[i].len() {
                    d = j + 1
                }
            } else {
                if c != 0 && d == 0 {
                    d = j;
                }
            }
        }
    }
    println!("{} {}", a, b);
    println!("{} {}", c, d);
}
