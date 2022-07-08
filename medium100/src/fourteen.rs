use proconio::input;
use proconio::marker::Chars;
pub fn main() {
    input! {
       h:usize,
       w:usize,
       c:[Chars;h]
    }
    let mut res = "Yes";
    for i in 1..h-1 {
        for j in 1..w-1 {
            if c[i][j] == '#' {
                if c[i][j - 1] != '#'
                    && c[i - 1][j] != '#'
                    && (i <=h && j < w && c[i][j + 1] != '#')
                    && (i < h && j <= w && c[i + 1][j] != '#')
                {
                    res = "No";
                    break;
                }
            }
        }
    }
    print!("{}", res);
}
