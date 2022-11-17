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
    let mut runlen = vec![];
    let mut cur = (s[0], 1);
    let sum = n * (n + 1) / 2;
    for i in 1..n {
        if s[i] == cur.0 {
            cur.1 += 1;
        } else {
            runlen.push(cur);
            cur = (s[i], 1);
        }
    }
    runlen.push(cur);
    let ans = sum - runlen.iter().map(|x| x.1 * (x.1 + 1) / 2).sum::<usize>();
    println!("{}", ans);
}
