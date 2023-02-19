use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
const MOD: usize = 998244353;

#[fastout]
pub fn main() {
    input! {
        n: usize,
        m: usize,
        k: i64,
        a: [Usize1; m],
    }

    let mut g = vec![vec![]; n];
    for i in 0..n - 1 {
        input! {
            u: Usize1,
            v: Usize1,
        }
        g[u].push((v, i));
        g[v].push((u, i));
    }

    let mut cnt = vec![0; n - 1];
    for i in 0..m - 1 {
        dfs(a[i], a[i + 1], n, &g, &mut cnt);
    }

    let sc: usize = cnt.iter().sum();
    if k + (sc as i64) < 0 {
        println!("0");
        return;
    }
    if (k + (sc as i64)) % 2 == 1 {
        println!("0");
        return;
    }
    let t = ((k + (sc as i64)) as usize) / 2;
    let mut dp = vec![0; t + 1];
    dp[0] = 1;
    for &x in &cnt {
        for i in (x..t + 1).rev() {
            dp[i] = (dp[i] + dp[i - x]) % MOD;
        }
    }
    println!("{}", dp[t]);
}

fn dfs(
    v: usize,
    goal: usize,
    p: usize,
    g: &Vec<Vec<(usize, usize)>>,
    cnt: &mut Vec<usize>,
) -> bool {
    if v == goal {
        return true;
    }
    for &(u, i) in &g[v] {
        if u == p {
            continue;
        }
        if dfs(u, goal, v, g, cnt) {
            cnt[i] += 1;
            return true;
        }
    }
    return false;
}
fn d() {
    input! {
    n:usize,a:[usize;n],b:[usize;n]
            }
    let max = 3010;
    let mo = 998244353;
    let mut dp = vec![vec![0; max]; max];
    dp[0][0] = 1;
    let mut cumsum = vec![0; max];
    for i in 0..n {
        cumsum[0] = dp[i][0];
        for last in 1..3001 {
            cumsum[last] = cumsum[last - 1] % mo + dp[i][last] % mo;
        }
        for j in a[i]..=b[i] {
            dp[i + 1][j] += cumsum[j];
        }
    }
    let mut res = 0;
    for i in 0..3001 {
        res += dp[n][i] % mo;
        res %= mo;
    }
    println!("{}", res);
}
