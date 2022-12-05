use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        mut k: usize,
    }
    let mut ans = 1usize;
    let mut p = 2;
    while p * p <= k {
        let mut a = 0;
        while k % p == 0 {
            k /= p;
            a += 1;
        }

        let mut n = 0;
        while a > 0 {
            n += p;
            let mut x = n;
            while x % p == 0 {
                x /= p;
                a -= 1;
            }
        }
        ans = ans.max(n);
        p += 1;
    }
    ans = ans.max(k);
    println!("{}", ans);
}
fn c() {
    input! {
        k: Chars,
        t: Chars,
    };

    for i in 0..k.len() {
        if k[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }
    println!("{}", k.len() + 1);
}
