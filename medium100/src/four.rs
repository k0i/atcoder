use proconio::{input, marker::Chars};

pub fn main() {
    input! {mut s:Chars}
    loop {
        s.pop();
        if s[..s.len() / 2] == s[s.len() / 2..] {
            print! {"{}",s.len()};
            break;
        }
    }
}
