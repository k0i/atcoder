use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
    n:usize,
    uvw:[(Usize1,Usize1,i64);n-1]
        }

    let mut g = vec![vec![]; n];
    let mut ans = vec![-1; n];

    for (u, v, w) in uvw {
        g[u].push((v, w));
        g[v].push((u, w));
    }
    ans[0] = 0;
    dfs(&g, &mut ans, 0, 0);

    for a in ans {
        println!("{}", a);
    }
}

fn dfs(g: &Vec<Vec<(usize, i64)>>, ans: &mut Vec<i64>, start: usize, current: usize) {
    for i in g[start].iter() {
        if ans[i.0] != -1 {
            continue;
        }
        if i.1 % 2 == 0 {
            ans[i.0] = ans[start];
        } else {
            ans[i.0] = match ans[start] {
                0 => 1,
                1 => 0,
                _ => unreachable!(),
            }
        }
        dfs(g, ans, i.0, current);
    }
}

fn c() {
    input! {
    n:usize,k:usize
        }
    let mut res: f64 = 0.0;
    for i in 1..=n {
        let mut prob = 1.0 / n as f64;
        if i >= k {
            res += prob;
            continue;
        }
        let mut j = 0;
        while i * 2usize.pow(j) < k {
            j += 1;
            prob *= 1.0 / 2.0;
        }
        res += prob;
    }
    println!("{}", res);
}
