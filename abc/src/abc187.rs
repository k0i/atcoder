use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut ab:[(usize,usize);n],
        }
    let mut sum = vec![];
    let mut aoki = 0;
    for i in ab.iter() {
        sum.push(i.0 * 2 + i.1);
        aoki += i.0;
    }
    sum.sort();
    sum.reverse();
    let mut res = 0;
    let mut takahashi = 0;
    for i in sum.iter() {
        takahashi += i;
        res += 1;
        if takahashi > aoki {
            break;
        }
    }
    println!("{}", res);
}
