use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize
        }
    let mut a = vec![];
    for i in 0..n {
        input! {
        line:Chars
        }
        a.push(line);
    }

    let mut res = "No";
    'outer: for i in 0..n {
        for j in 0..n {
            if i + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if a[i + k][j] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    res = "Yes";
                    break 'outer;
                }
            }
            if j + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if a[i][j + k] == '#' {
                        cnt += 1
                    };
                }
                if cnt >= 4 {
                    res = "Yes";
                    break 'outer;
                }
            }
            if (i + 5 < n) && (j + 5 < n) {
                let mut cnt = 0;
                for k in 0..6 {
                    if a[i + k][j + k] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    res = "Yes";
                    break 'outer;
                }
            }
            if i >= 5 && j + 5 < n {
                let mut cnt = 0;
                for k in 0..6 {
                    if a[i - k][j + k] == '#' {
                        cnt += 1;
                    }
                }
                if cnt >= 4 {
                    res = "Yes";
                    break 'outer;
                }
            }
        }
    }
    println!("{}", res);
}
