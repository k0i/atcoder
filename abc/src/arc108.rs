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
    let mut ans = vec![];

    for i in 0..n {
        ans.push(s[i]);
        if ans.len() >= 3 {
            if ans[ans.len() - 3..] == ['f', 'o', 'x'] {
                ans.pop();
                ans.pop();
                ans.pop();
            }
        }
    }
    println!("{}", ans.len());
}
