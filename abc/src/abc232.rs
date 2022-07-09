#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
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
