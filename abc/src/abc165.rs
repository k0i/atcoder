use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

fn dfs(
    n: usize,
    abcd: &[(usize, usize, i64, i64)],
    x: &mut [i64],
    i: usize,
    l: i64,
    r: i64,
    ans: &mut i64,
) {
    if i == n {
        let mut sum = 0;
        for (a, b, c, d) in abcd {
            if x[*b] - x[*a] == *c {
                sum += *d;
            }
        }
        *ans = (*ans).max(sum);
        return;
    }
    for j in l..r {
        x[i] = j;
        dfs(n, abcd, x, i + 1, j, r, ans);
    }
}
#[fastout]
pub fn main() {
    d()
}
fn c() {
    input! {
        n: usize,
        m: i64,
        q: usize,
        abcd: [(Usize1, Usize1, i64, i64); q],
    }
    let mut x = vec![0; 10];
    let mut ans = 0;
    dfs(n, &abcd, &mut x, 0, 1, m + 1, &mut ans);
    println!("{}", ans);
}

fn d() {
    input! {
    a:usize,
    b:usize,
    n:usize
        }

    let c = |a: usize, b: usize, x: usize| -> usize { (a * x) / b - a * (x / b) };
    let mut ans = 0;
    ans = std::cmp::max(ans, c(a, b, n));
    if b - 1 <= n {
        ans = std::cmp::max(ans, c(a, b, b - 1));
    }

    println!("{}", ans);
}
