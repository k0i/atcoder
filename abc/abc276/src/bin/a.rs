use proconio::{fastout, input, marker::Chars};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }

    let mut res = -1;
    for i in 0..s.len() {
        if s[i] == 'a' {
            res = (i + 1) as i32;
        }
    }
    println!("{}", res);
}
