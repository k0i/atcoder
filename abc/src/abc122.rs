use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:usize,
    q:usize,
    s:Chars
        }
    let mut cumsum = vec![];
    let mut ok = vec![];
    for i in 0..n - 1 {
        ok.push(s[i] == 'A' && s[i + 1] == 'C');
    }
    cumsum.push(0);
    for i in 0..n - 1 {
        cumsum.push(cumsum[i] + ok[i] as usize);
    }
    for _ in 0..q {
        input! {
            l:Usize1,
            r:Usize1,
        }
        println!("{}", cumsum[r] - cumsum[l]);
    }
}
