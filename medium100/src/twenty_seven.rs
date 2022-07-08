use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
        n:usize,
        mut s:Chars,
        mut t:Chars
    }
    for i in 0..n {
        if s[i..].to_vec() == t[..n - i].to_vec() {
            println!("{}", n + i);
            break;
        }
        if i == n - 1 {
            println!("{}", n * 2);
        }
    }
}
