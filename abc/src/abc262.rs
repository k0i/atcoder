use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[usize;n]
        }
    let mut temp = 0u64;

    for i in 0..n {
        a[i] -= 1;
        if a[i] == i {
            temp += 1;
        }
    }
    let mut res = temp * (temp - 1) / 2;
    for i in 0..n {
        if a[i] > i && a[a[i]] == i {
            res += 1;
        }
    }
    println!("{}", res);
}
