use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    t:usize
        }
    let r = 10000000;
    let mut sieve = vec![true; r + 1];
    sieve[0] = false;
    sieve[1] = false;
    for i in 2..r + 1 {
        if sieve[i] {
            for j in 2..r / i + 1 {
                sieve[i * j] = false;
            }
        }
    }
    let mut prime = HashSet::new();
    for i in 2..r + 1 {
        if sieve[i] {
            prime.insert(i as u128);
        }
    }
    'outer: for _ in 0..t {
        input! {
        n:u128
        }
        for j in prime.iter() {
            if n % j != 0 {
                continue;
            }
            let mut k = n / j;
            if k % j == 0 {
                continue;
            }
        }
    }
}
