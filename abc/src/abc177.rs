use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[u128;n]
        }
    let modn = 10_u128.pow(9) + 7;
    let mut res = 0;
    let mut a_sum = a.iter().sum::<u128>();
    for i in 0..n {
        a_sum -= a[i];
        res += a[i] * a_sum;
        res %= modn;
    }
    println!("{}", res % modn);
}
