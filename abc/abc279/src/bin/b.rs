use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    s:Chars,
    t:Chars
        }
    let mut ans = "No";
    'outer: for i in 0..s.len() {
        if s[i] == t[0] {
            for j in 0..t.len() {
                if i + j >= s.len() {
                    continue 'outer;
                }
                if s[i + j] != t[j] {
                    continue 'outer;
                }
                if j == t.len() - 1 {
                    ans = "Yes";
                    break;
                }
            }
        }
    }
    println!("{}", ans);
}
