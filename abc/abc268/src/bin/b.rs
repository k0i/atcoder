use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars,
    t:Chars
        }
    let mut res = "No";

    let mut len = 0;
    if s.len() > t.len() {
        println!("{}", res);
        return;
    }
    loop {
        if len == s.len() {
            res = "Yes";
            break;
        }
        if s[len] != t[len] {
            break;
        }
        len += 1;
    }
    println!("{}", res);
}
