use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
       n:usize,
       s:Chars,
    }
    let mut res = 0;
    for i in 0..n {
        let mut temp = Vec::new();
        let left = &s[..i];
        let right = &s[i..];
        for j in 0..left.len() {
            if right.iter().find(|r| r == &&left[j]).is_some()
                && temp.iter().find(|r| r == &&left[j]).is_none()
            {
                temp.push(left[j]);
            }
        }
        if temp.len() > res {
            res = temp.len();
        }
    }
    print!("{}", res);
}
