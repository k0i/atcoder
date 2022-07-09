#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {h1:i64,h2:i64,h3:i64,w1:i64,w2:i64,w3:i64}
    let mut res = 0;
    for i in 1..=30 {
        for j in 1..=30 {
            for k in 1..=30 {
                for l in 1..=30 {
                    let first_row = h1 - i - j;
                    if first_row <= 0 {
                        continue;
                    }
                    let second_row = h2 - k - l;
                    if second_row <= 0 {
                        continue;
                    }
                    let first_col = w1 - i - k;
                    if first_col <= 0 {
                        continue;
                    }
                    let second_col = w2 - j - l;
                    if second_col <= 0 {
                        continue;
                    }
                    let last_row = h3 - first_col - second_col;
                    let last_col = w3 - first_row - second_row;
                    if last_row > 0 && last_col > 0 && last_row == last_col {
                        res += 1
                    }
                }
            }
        }
    }
    println!("{}", res);
}
