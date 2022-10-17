use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    s:Chars
        }
    let mut res = 300001;
    let mut w = 0;
    let mut past_w = 0;
    for i in s.iter() {
        if let 'W' = i {
            w += 1;
        }
    }
    let e = n - w;
    let mut past_e = 0;
    for i in 0..n {
        if s[i] == 'W' {
            res = std::cmp::min(res, past_w + e - past_e);
        } else {
            res = std::cmp::min(res, past_w + e - past_e - 1);
        }
        if let 'W' = s[i] {
            past_w += 1;
        } else {
            past_e += 1;
        }
    }
    println!("{}", res);
}
