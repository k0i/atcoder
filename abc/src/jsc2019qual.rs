use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        (n, k): (usize,usize),
        a: [usize; n]
    }

    let modp = 1_000_000_000 + 7;
    let kk = ((k * (k - 1)) / 2) % modp;

    let internal: usize = (0..n)
        .map(|i| (&a[(i + 1)..]).iter().filter(|x| **x < a[i]).count())
        .sum::<usize>()
        % modp;

    let external: usize = (0..n)
        .map(|i| (&a[..]).iter().filter(|x| **x < a[i]).count())
        .sum::<usize>()
        % modp;

    let internal_sum = (0..internal).fold(0, |acc, _| (acc + k) % modp);
    let external_sum = (0..external).fold(0, |acc, _| (acc + kk) % modp);

    println!("{}", (internal_sum + external_sum) % modp);
}
