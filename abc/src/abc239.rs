#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input as ip,
    marker::{Bytes, Chars, Isize1, Usize1 as U1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    ip! {
        n: usize,
        q: usize,
        x: [u32; n],
        ab: [(U1, U1); n - 1],
        vk: [(U1, U1); q]
    }

    let mut t = vec![vec![]; n];
    for &(a, b) in &ab {
        t[a].push(b);
        t[b].push(a);
    }

    fn dfs(
        t: &Vec<Vec<usize>>,
        x: &Vec<u32>,
        y: &mut Vec<Vec<u32>>,
        i: usize,
        p: usize,
    ) -> Vec<u32> {
        let mut c = vec![x[i]];
        for &j in &t[i] {
            if j != p {
                c.append(&mut dfs(t, x, y, j, i));
            }
        }
        c.sort_by(|a, b| b.cmp(&a));
        c.resize(20, 0);
        y[i] = c.clone();
        c
    }

    let mut y = vec![vec![]; n];
    dfs(&t, &x, &mut y, 0, 0);

    for &(v, k) in &vk {
        println!("{}", y[v][k]);
    }
}
fn d() {
    ip! {
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
