use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[fastout]
pub fn main() {
    input!(n: usize, m: usize, xy: [(Usize1, Usize1); m],);

    let mut keep = vec![1; n];
    let mut red = vec![0; n];
    red[0] = 1;

    for &(x, y) in xy.iter() {
        if red[x] == 1 {
            red[y] = 1;
        }
        keep[x] -= 1;
        keep[y] += 1;
        if keep[x] == 0 {
            red[x] = 0;
        }
    }

    println!("{}", red.iter().sum::<i64>());
}
