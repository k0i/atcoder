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
