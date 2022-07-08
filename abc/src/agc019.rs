#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {q:u64,h:u64,s:u64,d:u64,n:u64};
    let onev = vec![4 * q, 2 * h, s];
    let twov = vec![8 * q, 4 * h, 2 * s, d];
    let one = onev.into_iter().min().unwrap();
    let two = twov.into_iter().min().unwrap();
    if n % 2 == 0 {
        println!("{}", n / 2 * two);
    } else {
        println!("{}", n / 2 * two + one);
    }
}
