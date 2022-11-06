use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut a:[usize;n],
        }
    a.sort();
    let mut g = 0;
    for i in a.iter() {
        g = gcd(g, *i);
    }
    for i in 0..n {
        a[i] /= g;
    }
    let mut ans = 0;
    for j in 0..n {
        let mut i = a[j];
        while i % 2 == 0 {
            i /= 2;
            ans += 1;
        }
        while i % 3 == 0 {
            i /= 3;
            ans += 1;
        }
        if i != 1 {
            println!("-1");
            return;
        }
    }
    println!("{}", ans);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a < b {
        std::mem::swap(&mut a, &mut b);
    }
    if b == 0 {
        return a;
    }
    while a % b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    b
}
