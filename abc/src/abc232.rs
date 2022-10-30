use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    input! {
    n:usize,
    m:usize,
    a:[(Usize1,Usize1);m],
    b:[(Usize1,Usize1);m],
    }
    let mut perm: Vec<_> = (0..n).permutations(n).collect();
    let mut tak = vec![vec![false; n]; n];
    let mut aoki = vec![vec![false; n]; n];
    for (from, to) in a {
        tak[from][to] = true;
        tak[to][from] = true;
    }
    for (from, to) in b {
        aoki[from][to] = true;
        aoki[to][from] = true;
    }
    while !perm.is_empty() {
        let mut ok = true;
        let p = perm.pop().unwrap();
        for i in 0..n {
            for j in 0..n {
                if tak[i][j] != aoki[p[i]][p[j]] {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
    }
    println!("No");
}

fn d() {
    input! {h:usize,w:usize,grid:[Chars;h]}
    let neg = -100000000;
    let mut dp = vec![vec![neg; w]; h];
    dp[0][0] = 1;
    for i in 0..h {
        for j in 0..w {
            if i == 0 && j == 0 {
                continue;
            }
            if grid[i][j] == '.' {
                let above = if i == 0 { neg } else { dp[i - 1][j] };
                let left = if j == 0 { neg } else { dp[i][j - 1] };
                dp[i][j] = std::cmp::max(above, left) + 1;
            }
        }
    }
    let mut res = 0;
    for i in dp {
        for j in i {
            if j > res {
                res = j;
            }
        }
    }
    println!("{}", res);
}
