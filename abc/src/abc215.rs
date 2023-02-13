use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        s: Bytes,
    }

    let mut dp = vec![vec![vec![0; 11]; 1 << 10]; n + 1];
    let MOD = 998244353;
    dp[0][0][10] = 1;
    for i in 0..n {
        let c = (s[i] - b'A') as usize;
        for j in 0..1 << 10 {
            for k in 0..11 {
                dp[i + 1][j][k] += dp[i][j][k];
                dp[i + 1][j][k] %= MOD;
                if k == c && j >> k & 1 == 1 {
                    dp[i + 1][j][k] += dp[i][j][k];
                    dp[i + 1][j][k] %= MOD;
                } else if k != c && j >> c & 1 == 0 {
                    dp[i + 1][j | 1 << c][c] += dp[i][j][k];
                    dp[i + 1][j | 1 << c][c] %= MOD;
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..1 << 10 {
        for j in 0..10 {
            res += dp[n][i][j];
            res %= MOD;
        }
    }
    println!("{}", res);
}
fn d() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    };
    let mut hs = HashSet::new();
    for i in 0..n {
        let mut j = 1;
        while j * j <= a[i] {
            if a[i] % j == 0 {
                hs.insert(j);
                hs.insert(a[i] / j);
            }
            j += 1;
        }
    }
    hs.remove(&1);

    let mut ng = HashSet::new();
    for x in hs.into_iter() {
        let mut i = 1;
        while x * i <= m {
            ng.insert(x * i);
            i += 1;
        }
    }

    let mut res = vec![];
    for i in 1..=m {
        if !ng.contains(&i) {
            res.push(i);
        }
    }
    res.sort();
    println!("{}", res.len());
    for r in res.into_iter() {
        println!("{}", r);
    }
}
