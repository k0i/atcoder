use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[[usize;6];n],
        }
    let mut sum = vec![];
    for i in 0..n {
        let t: usize = a[i].iter().sum();
        sum.push(t);
    }

    let mut ans = 1;
    for i in 0..n {
        ans = ans * sum[i] % 1000000007
    }
    println!("{}", ans);
}
