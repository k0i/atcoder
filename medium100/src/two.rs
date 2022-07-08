use proconio::{input, marker::Chars};

pub fn main() {
    input! {s:Chars}
    let start = s.iter().position(|b| b.to_string() == "B");
    if let None = start {
        println!("{}", 0)
    } else {
        let mut white = 0;
        let mut res = 0;
        for i in start.unwrap()..s.len() {
            if s[i].to_string() == "W" {
                res += i - white - start.unwrap();
                white += 1;
            }
        }
        println!("{:?}", res)
    }
}
