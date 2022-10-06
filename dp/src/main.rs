use std::collections::VecDeque;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[allow(dead_code)]
fn update<T: PartialOrd>(value: T, target: &mut T, cond: std::cmp::Ordering) {
    if value.partial_cmp(target) == Some(cond) {
        *target = value;
    }
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Greater)
    };
}
#[allow(unused_macros)]
macro_rules! chmin {
    ($target:expr, $value:expr) => {
        update($value, &mut $target, std::cmp::Ordering::Less)
    };
}

pub fn main() {
    i()
}

fn i() {
    input! {
    n:usize,
    p:[f64;n]
    }
    let mut dp = vec![vec![0.0; n + 1]; n + 1];
    dp[0][0] = 1.0;
    for i in 1..n + 1 {
        for j in 0..=i {
            dp[i][j] = dp[i - 1][j] * (1.0 - p[i - 1]);
            if j > 0 {
                dp[i][j] += dp[i - 1][j - 1] * p[i - 1];
            }
        }
    }
    let mut ans = 0.0;
    for i in (n / 2 + 1)..n + 1 {
        ans += dp[n][i];
    }
    println!("{:?}", dp);
    println!("{}", ans);
}

fn h() {
    input! {
    h:usize,
    w:usize,
    s:[Chars;h],
    }
    let mut dp = vec![vec![0; w + 1]; h + 1];
    dp[1][1] = 1;

    for i in 1..=h {
        for j in 1..=w {
            if s[i - 1][j - 1] == '#' {
                continue;
            }
            dp[i][j] += dp[i - 1][j] + dp[i][j - 1];
            dp[i][j] %= 1000000007;
        }
    }
    println!("{}", dp[h][w] % 1000000007);
}

fn g() {
    input! {
    n:usize,
    m:usize,
    xy:[(Usize1,Usize1);m]
    }
    let mut graph = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for (x, y) in xy {
        graph[x].push(y);
        indeg[y] += 1;
    }
    let mut queue = VecDeque::new();
    let mut res = vec![];
    for i in 0..indeg.len() {
        if indeg[i] == 0 {
            queue.push_back(i);
        }
    }
    while let Some(i) = queue.pop_front() {
        res.push(i);
        for &j in &graph[i] {
            indeg[j] -= 1;
            if indeg[j] == 0 {
                queue.push_back(j);
            }
        }
    }
    let mut dp = vec![0; n];
    for &i in &res {
        for &j in &graph[i] {
            chmax!(dp[j], dp[i] + 1);
        }
    }
    println!("{}", dp.iter().max().unwrap());
}

fn f() {
    input! {
    s:Chars,
    t:Chars
        }
    let mut dp = vec![[0; 3001]; 3001];
    for i in 0..s.len() {
        for j in 0..t.len() {
            if s[i] == t[j] {
                chmax!(dp[i + 1][j + 1], dp[i][j] + 1);
            }
            chmax!(dp[i + 1][j + 1], dp[i + 1][j]);
            chmax!(dp[i + 1][j + 1], dp[i][j + 1]);
        }
    }
    let mut ans = String::new();
    let mut i = s.len();
    let mut j = t.len();
    while i > 0 && j > 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            ans.push(s[i - 1]);
            i -= 1;
            j -= 1;
        }
    }
    println!("{}", ans.chars().rev().collect::<String>());
}

fn b_distribute() {
    input! {n:usize,k:usize,h:[i64;n]}
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 0..n {
        for j in 1..=k {
            if i + j < n {
                chmin!(dp[i + j], dp[i] + (h[i] - h[i + j]).abs());
            }
        }
    }
    println!("{}", dp.iter().last().unwrap());
}

fn e() {
    input! {n:usize,w:i64,a:[(i64,i64);n]}
    let mut dp = vec![vec![std::i64::MAX - 1000000000; 100001]; 101];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..100001 as usize {
            if j >= a[i].1 as usize {
                chmin!(dp[i + 1][j], dp[i][j - a[i].1 as usize] + a[i].0);
            }
            chmin!(dp[i + 1][j], dp[i][j]);
        }
    }
    let mut res = 0;
    for i in 0..100001 {
        if dp[n][i] <= w {
            res = i;
        }
    }
    println!("{}", res);
}

fn d() {
    input! {n:usize,w:i64,a:[(i64,i64);n]}
    let mut dp = vec![vec![0; w as usize + 1]; n + 1];
    for i in 0..n {
        for j in 0..=w as usize {
            if j >= a[i].0 as usize {
                chmax!(dp[i + 1][j], dp[i][j - a[i].0 as usize] + a[i].1);
            }
            chmax!(dp[i + 1][j], dp[i][j]);
        }
    }
    println!("{}", dp.iter().last().unwrap().iter().max().unwrap());
}

fn c() {
    input! {n:usize,r:[[i64;3];n]}
    let mut dp = vec![vec![0; 3]; n];
    for i in 0..n {
        for j in 0..3 {
            if i == 0 {
                dp[i][j] = r[i][j];
                continue;
            }
            for k in 0..3 {
                if j == k {
                    continue;
                }
                chmax!(dp[i][j], dp[i - 1][k] + r[i][j]);
            }
        }
    }
    let mut res = 0;
    for r in dp {
        res = std::cmp::max(res, r.into_iter().max().unwrap());
    }
    println!("{}", res);
}

fn b() {
    input! {n:usize,k:usize,h:[i64;n]}
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        for j in 1..=k {
            if i >= j {
                chmin!(dp[i], dp[i - j] + (h[i] - h[i - j]).abs());
            }
        }
    }
    println!("{}", dp.iter().last().unwrap());
}
fn a() {
    input! {
    n:usize,h:[i64;n],
        }
    let mut dp = vec![std::i64::MAX; n];
    dp[0] = 0;
    for i in 1..n {
        if i == 1 {
            dp[i] = (h[i] - h[i - 1]).abs();
            continue;
        }
        chmin!(dp[i], dp[i - 1] + (h[i] - h[i - 1]).abs());
        chmin!(dp[i], dp[i - 2] + (h[i] - h[i - 2]).abs());
    }
    println!("{}", dp.iter().last().unwrap());
}
