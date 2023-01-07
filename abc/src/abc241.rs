use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut sum = vec![0];
    let mut seen_at = vec![n; n];
    seen_at[0] = 0;
    let mut loop_first = n;

    for i in 0..n {
        let next_sum = sum[i] + a[sum[i] % n];
        sum.push(next_sum);
        if seen_at[next_sum % n] < n {
            loop_first = seen_at[next_sum % n];
            break;
        }
        seen_at[next_sum % n] = i + 1;
    }

    if k < sum.len() {
        println!("{}", sum[k]);
        return;
    }
    let loop_interval = sum.len() - loop_first - 1;
    let inc_per_loop = sum.last().unwrap() - sum[loop_first];
    let q = (k - loop_first) / loop_interval;
    let r = (k - loop_first) % loop_interval;
    let ans = q * inc_per_loop + sum[loop_first + r];
    println!("{}", ans);
}

fn c() {
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
