use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    a()
}

fn a() {
    input! {
    n:usize,
    a:usize,
    b:usize
        }
    if a > b || (n == 1 && a != b) {
        println!("0");
        return;
    }
    let min = a * (n - 1) + b;
    let max = b * (n - 1) + a;
    println!("{}", max - min + 1);
}

fn b() {
    input! {
    s:Chars
        }
    let mut ans = 0;
    for i in 0..s.len() {
        match s[i] {
            'U' => {
                ans += s.len() - i - 1 + (i * 2);
            }
            _ => {
                ans += i + (s.len() - i - 1) * 2;
            }
        }
    }
    println!("{}", ans);
}
