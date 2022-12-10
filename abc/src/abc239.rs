use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    a:usize,
    b:usize,
    c:usize,
    d:usize,
        }
    let primes = prime_sieve_up_to200();
    let mut hash = HashSet::new();
    for i in primes {
        hash.insert(i);
    }
    'outer: for i in a..=b {
        for j in c..=d {
            if hash.contains(&(i + j)) {
                continue 'outer;
            }
        }
        println!("Takahashi");
        return;
    }
    println!("Aoki");
}

fn prime_sieve_up_to200() -> Vec<usize> {
    let mut is_prime = vec![true; 200];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..200 {
        if is_prime[i] {
            for j in 2.. {
                if i * j >= 200 {
                    break;
                }
                is_prime[i * j] = false;
            }
        }
    }
    let mut primes = vec![];
    for i in 0..200 {
        if is_prime[i] {
            primes.push(i);
        }
    }
    primes
}
