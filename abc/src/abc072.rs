use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    p:[usize;n]
        }
    let mut ch = vec![false; n];
    for i in 0..n {
        if p[i] == i + 1 {
            ch[i] = true;
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if ch[i] {
            if i + 1 < n {
                ch[i + 1] = false;
            }
            ans += 1;
        }
    }
    println!("{}", ans);
}
