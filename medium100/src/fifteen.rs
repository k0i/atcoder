use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
       s:Chars
    }

    'outer:for i in 0..s.len() {
        for j in i..s.len() - 1 {
            let mut temp = s.clone();
            temp.drain(i..j);
            let res: String = temp.into_iter().collect();
            if res == "keyence" {
                print!("YES");
                break 'outer;
            }
        }
        if i == s.len() - 1 {
            print!("NO");
        }
    }
}
