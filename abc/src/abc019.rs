#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    b()
}

fn b() {
    input! {s:Chars}
    let mut temp = s[0];
    let mut cnt = 1;
    let mut ans = vec![];
    for i in 1..s.len() {
        if s[i] == temp {
            cnt += 1;
        } else {
            ans.push(format!("{}{}", temp, cnt));
            cnt = 1;
            temp = s[i];
        }
        if i == s.len() - 1 {
            ans.push(format!("{}{}", temp, cnt));
        }
    }
    println!("{}", ans.join(""));
}
