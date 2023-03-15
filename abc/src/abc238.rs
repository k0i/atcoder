use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        q: usize,
        lr: [(Usize1,usize);q],
    }

    let mut ad = vec![vec![]; n + 1];
    for &(l, r) in lr.iter() {
        ad[l].push(r);
        ad[r].push(l);
    }

    let mut visited = vec![false; n + 1];
    let start = 0;
    let mut todo = VecDeque::new();
    todo.push_back(start);
    loop {
        if let Some(i) = todo.pop_front() {
            if visited[i] {
                continue;
            }
            visited[i] = true;
            for &j in ad[i].iter() {
                todo.push_back(j);
            }
        } else {
            break;
        }
    }

    if visited[n] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn d() {
    input! {
        t: usize,
        ts: [(usize, usize); t],
    };
    for &(a, s) in ts.iter() {
        if a * 2 > s {
            println!("No");
        } else {
            let x = s - (a << 1);
            if x & a == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}

fn c() {
    input! {
        n: u128
    }
    const MOD: u128 = 998244353;
    let mut ans = 0;
    let f = |n: u128| {
        if n % 2 == 0 {
            n / 2 % MOD * (n + 1) % MOD
        } else {
            n % MOD * (n + 1) / 2 % MOD
        }
    };
    for i in 0.. {
        let l = 10u128.pow(i);
        let r = (10u128.pow(i + 1) - 1).min(n);
        ans += f(r - l + 1);
        ans %= 998244353;
        if r == n {
            break;
        }
    }
    println!("{}", ans);
}
