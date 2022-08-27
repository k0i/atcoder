use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars
    }
    if s.len() == 1 {
        println!("{}", s[0]);
        return;
    }
    let mid = s.len() / 2;
    println!("{}", s[mid]);
}
