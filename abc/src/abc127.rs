use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    m:usize,
    mut a:[usize;n],
     mut    bc:[(usize,usize);m]
        }
    let mut res = 0;
    a.sort();
    bc.sort_by(|x, y| y.1.cmp(&x.1));
    let mut k = 0;
    let mut i = 0;
    while i < m {
        for j in 0..bc[i].0 {
            if k >= n {
                break;
            }
            res += std::cmp::max(a[k], bc[i].1);
            k += 1;
        }
        i += 1;
    }
    if k <= n {
        for i in k..n {
            res += a[i];
        }
    }
    println!("{}", res);
}
