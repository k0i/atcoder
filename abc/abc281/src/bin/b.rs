use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let upper_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if s.len() != 8 {
        println!("No");
        return;
    }
    if s[1].to_digit(10).unwrap_or(0) == 0 {
        println!("No");
        return;
    }
    for i in 2..=6 {
        if s[i].to_digit(10).unwrap_or(100) == 100 {
            println!("No");
            return;
        }
    }
    if !upper_alphabet.contains(s[0]) || !upper_alphabet.contains(s[7]) {
        println!("No");
        return;
    }
    println!("Yes");
}
