#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{fastout, input, source::line::LineSource};
use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, BufReader, Write};
#[fastout]
pub fn main() {
    proconio::input! {
        n: usize, m: usize, k: usize,
        s: usize, t: usize, x: usize,
        v: [(usize, usize); m],
    };

    const MOD: usize = 998244353;

    let mut g = vec![Vec::new(); n + 1];
    v.iter().for_each(|&root| {
        g[root.0].push(root.1);
        g[root.1].push(root.0);
    });

    let mut dp = vec![vec![vec![0usize; 2]; n + 1]; k + 1]; // dp[K][N][%2]
    dp[0][s][0] = 1;
    for i in 0..k {
        for j in 1..=n {
            for &l in g[j].iter() {
                for ll in 0..2 {
                    let rev = if x == l { 1 } else { 0 };
                    dp[i + 1][l][(ll + rev) % 2] += dp[i][j][ll];
                    dp[i + 1][l][(ll + rev) % 2] %= MOD;
                }
            }
        }
    }

    println!("{}", dp[k][t][0]);
}
fn d() {
    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin.lock()));
    input! {
          from &mut source,
    n:usize
        }
    let mut av = HashSet::new();
    for i in 1..=n * 2 + 1 {
        av.insert(i);
    }
    loop {
        let a = *av.iter().next().unwrap();
        println!("{}", a);
        stdout().flush().unwrap();
        av.remove(&a);
        input! {
        from &mut source,
        b:usize
        }
        if b == 0 {
            return;
        }
        av.remove(&b);
    }
}
