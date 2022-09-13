use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    x:Chars
        }
    let mut res = "YES";
    let mut i = 0;
    while i < x.len() {
        if x[i] == 'o' || x[i] == 'k' || x[i] == 'u' {
            i += 1;
            continue;
        }
        if x[i] == 'c' && i + 1 < x.len() && x[i + 1] == 'h' {
            i += 2;
            continue;
        }
        res = "NO";
        break;
    }
    println!("{}", res);
}
