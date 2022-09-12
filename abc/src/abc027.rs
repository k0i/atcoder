use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n]
         }

    let mut res = 0;
    let mut b = vec![false; n];
    let mut sum = 0;
    for i in a.iter() {
        sum += i;
    }
    if sum % n != 0 {
        println!("-1");
        return;
    }
    let avg = sum / n;
    let mut i = 0;
    let mut cur = 0;
    for i in 0..(n - 1) {
        cur += a[i];
        if cur == avg * (i + 1) && (n - i - 1) * avg == sum - cur {
            continue;
        } else {
            res += 1;
        }
    }
    println!("{:?}", res);
}
