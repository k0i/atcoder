use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    t:usize,
    n:usize,
    a:[usize;n],
    m:usize,
    b:[usize;m],
        }
    let mut res = "no";
    let mut pos = 0;

    let mut cnt = 0;
    for i in 0..m {
        for j in pos..n {
            if b[i] >= a[j] && b[i] <= a[j] + t {
                pos = j + 1;
                cnt += 1;
                break;
            }
            if j == n - 1 {
                println!("{}", res);
                return;
            }
        }
        if i == m - 1 && cnt == m {
            res = "yes";
        }
    }
    println!("{}", res);
}
