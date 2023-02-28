use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n: usize, m: usize, mut a: [i64; n+1], mut c: [i64; n+m+1]}
    a.reverse();
    c.reverse();

    let mut b = vec![];
    for i in 0..=m {
        let r = c[i] / a[0];
        b.push(r);
        for j in 0..=n {
            c[i + j] -= a[j] * r;
        }
    }
    b.reverse();
    println!("{}", b.iter().join(" "));
}

fn c() {
    input! {
    n:usize,
    k:i64,
    a:[i64;n],
    b:[i64;n]
        }
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];
    dp[0] = true;
    ep[0] = true;
    for i in 1..n {
        if dp[i - 1] {
            if (a[i] - a[i - 1]).abs() <= k {
                dp[i] = true;
            }
            if (b[i] - a[i - 1]).abs() <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] {
            if (b[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (b[i] - b[i - 1]).abs() <= k {
                ep[i] = true;
            }
        }
    }

    if dp[n - 1] || ep[n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
