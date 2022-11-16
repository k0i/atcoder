use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut n:usize
         }

    let mut i = 2;
    let mut l = 0;
    while i * i <= n {
        if n % i == 0 {
            let mut cnt = 0;
            while n % i == 0 {
                n /= i;
                cnt += 1;
            }
            l += cnt;
        }
        i += 1;
    }
    if n != 1 {
        l += 1;
    }

    let mut ans = 0;
    let mut temp = 1;

    while temp < l {
        temp *= 2;
        ans += 1;
    }
    println!("{}", ans);
}
