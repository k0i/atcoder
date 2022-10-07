use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut s:usize,
    ab:[(usize,usize);n]
        }
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=s {
            if dp[i][j] {
                if j + a <= s {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= s {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }
    if dp[n][s] {
        println!("Yes");
    } else {
        println!("No");
        return;
    }
    let mut t = vec!['H'; n];
    for i in (0..n).rev() {
        if s >= ab[i].0 && dp[i][s - ab[i].0] {
            t[i] = 'H';
            s -= ab[i].0;
        } else {
            t[i] = 'T';
            s -= ab[i].1;
        }
    }
    let ans = t.into_iter().collect::<String>();
    println!("{}", ans);
}
