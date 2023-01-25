use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n:usize,
        xy:[(i64,i64);n]
    };

    let mut zs = vec![];
    let mut ws = vec![];
    for (x, y) in xy {
        zs.push(x + y);
        ws.push(x - y);
    }

    let z_min = zs.iter().min().unwrap();
    let z_max = zs.iter().max().unwrap();
    let w_min = ws.iter().min().unwrap();
    let w_max = ws.iter().max().unwrap();

    println!("{}", std::cmp::max(z_max - z_min, w_max - w_min));
}
fn d() {
    input! {
        s: usize,
    }

    let mut dp = vec![0; s + 1];
    let m = 1000000007;
    dp[0] = 1;

    for i in 3..=s {
        dp[i] = (dp[i - 1] + dp[i - 3]) % m;
    }
    println!("{}", dp[s]);
}
fn c() {
    input! {
    n:i64
    }
    let m = 10i64.pow(9) + 7;
    let ans = mp(10, n) - 2 * mp(9, n) + mp(8, n) + 2 * m;
    println!("{:?}", ans % m);
}

fn mp(x: i64, n: i64) -> i64 {
    let mut ans = 1;
    for _i in 0..n {
        ans = (ans * x) % (10i64.pow(9) + 7);
    }
    ans
}
