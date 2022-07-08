#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {a:u64,b:u64,c:u64}
    let ln = gcd(gcd(a, b), c);
    let ans = a / ln + b / ln + c / ln - 3;
    println!("{:?}", ans);
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    let mut res = b;
    while a % b != 0 {
        res = a % b;
        a = b;
        b = res;
    }
    res
}
