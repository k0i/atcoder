use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
       n: usize,
       q: usize,
       a: [usize; n],
       k: [Usize1; q],
    }
    let mut x = vec![];
    let mut y = vec![];
    let mut i = 0;
    while i < n {
        if i == 0 && a[0] != 1 {
            x.push(1);
            y.push(0);
        }
        if i == n - 1 || a[i] + 1 != a[i + 1] {
            x.push(a[i] + 1);
            y.push(a[i] - i - 1);
        }
        i += 1;
    }

    for ki in k {
        let i = y
            .binary_search_by(|&x| {
                if x <= ki {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i)
            - 1;
        println!("{}", x[i] + ki - y[i]);
    }
}
