#[allow(unused_imports)]
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
        s: Chars,
        q: usize,
        tk: [(usize, usize);q],
    }
    for &(t, k) in tk.iter() {
        let mut k = k - 1;
        let mut si = 0;
        if t <= 60 {
            let b = 1 << t;
            si = k / b;
            k %= b;
        }
        let r = k.count_ones() as usize;
        let l = t - r;
        let x = l + r * 2 + (s[si] as usize - 'A' as usize);
        let ans = ('A' as usize + (x % 3)) as u8 as char;
        println!("{}", ans);
    }
}

fn c() {
    input! {n:usize}
    let mut dp = vec![vec![1; 9]; n];
    for i in 1..n {
        for j in 0..9 {
            match j {
                0 => dp[i][j] = dp[i - 1][j] + dp[i - 1][j + 1],
                8 => dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1],
                _ => dp[i][j] = dp[i - 1][j] + dp[i - 1][j + 1] + dp[i - 1][j - 1],
            }
            dp[i][j] %= 998244353;
        }
    }
    println!("{:?}", dp.last().unwrap().iter().sum::<usize>() % 998244353);
}
