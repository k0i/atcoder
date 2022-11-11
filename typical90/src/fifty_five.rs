use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
pub fn main() {
    input! {
    n:usize,
    p:usize,
    q:usize,
    a:[usize;n]
        }

    let mut res = 0;
    for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                for l in k + 1..n {
                    for m in l + 1..n {
                        let mut prod = 1;
                        prod *= a[i];
                        prod %= p;
                        prod *= a[j];
                        prod %= p;
                        prod *= a[k];
                        prod %= p;
                        prod *= a[l];
                        prod %= p;
                        prod *= a[m];
                        prod %= p;

                        if prod == q {
                            res += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", res);
}
