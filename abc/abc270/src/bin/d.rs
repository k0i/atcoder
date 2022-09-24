use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {mut n:usize,k:usize,a:[usize;k]}
    let mut takahashi = 0;
    let mut len = k - 1;
    loop {
        if n == 0 {
            break;
        }
        let mut max = a[len];
        if n < max {
            len -= 1;
            continue;
        }
        takahashi += max;
        n -= max;
        if n == 0 {
            break;
        }
        if n >= max {
            n -= max;
            continue;
        }
        len -= 1;
        let mut aoki_max = a[len];
        while aoki_max > n {
            len -= 1;
            aoki_max = a[len];
        }
        len += 1;
        n -= aoki_max;
    }
    println!("{}", takahashi);
}
