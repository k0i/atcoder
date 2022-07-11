#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,k:usize,mut x:[(u128,u128);n]}
    x.sort_by(|a, b| a.0.cmp(&b.0));
    let mut res = k as u128;
    for i in 0..n {
        let (a, b) = x[i];
        if a <= res {
            res += b;
        }
    }
    println!("{:?}", res);
}
