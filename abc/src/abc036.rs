use proconio::marker::*;
use proconio::*;
pub fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }
    let mut b = a.clone();
    b.sort();
    b.dedup();

    let res = a
        .iter()
        .map(|&x| b.binary_search(&x).unwrap())
        .for_each(|x| println!("{}", x));
}
