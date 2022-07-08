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
       n: usize,
       k: i64,
       a: [i64;n],
       b: [i64;n]
    }
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];
    dp[0] = true;
    ep[0] = true;

    for i in 1..n {
        if dp[i - 1] == true {
            if (a[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (a[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] == true {
            if (b[i - 1] - a[i]).abs() <= k {
                dp[i] = true;
            }
            if (b[i - 1] - b[i]).abs() <= k {
                ep[i] = true;
            }
        }
    }
    if dp[n - 1] == true || ep[n - 1] == true {
        println!("Yes");
    } else {
        println!("No");
    }
}
