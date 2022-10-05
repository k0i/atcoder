use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:usize
        }
    let mut sum = vec![0; n + 2];
    let mut a: Vec<_> = (0..n + 1).collect();
    for i in 0..n + 1 {
        sum[i + 1] = sum[i] + a[i];
    }
    let mut res = 0;
    for i in k..=n + 1 {
        let min = sum[i];
        let max = sum[n + 1] - sum[n + 1 - i];
        let x = max - min + 1;
        res += x;
        res %= 1000000007;
    }
    println!("{:?}", res % 1000000007);
}
