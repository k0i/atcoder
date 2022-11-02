use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    lr:[((usize,usize),(usize,usize));n],
        }

    let mut g = vec![vec![0; 1001]; 1001];
    let mut ans = vec![0; n];

    for i in lr {
        let (l, r) = i;
        g[l.0][l.1] += 1;
        g[r.0][r.1] += 1;
        g[l.0][r.1] -= 1;
        g[r.0][l.1] -= 1;
    }
    for i in 0..1001 {
        for j in 0..1000 {
            g[i][j + 1] += g[i][j];
        }
    }
    for i in 0..1000 {
        for j in 0..1001 {
            g[i + 1][j] += g[i][j];
        }
    }
    for i in 0..1001 {
        for j in 0..1001 {
            if g[i][j] > 0 {
                ans[g[i][j] - 1] += 1;
            }
        }
    }
    for i in ans {
        println!("{}", i);
    }
}
