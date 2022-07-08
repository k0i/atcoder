use proconio::input;
pub fn main() {
    input! {
       n:usize,m:usize,mut x:[i64;m]
    }
    let mut diff = vec![0; m - 1];
    x.sort_by(|a, b| b.cmp(a));
    for i in 1..m {
        diff[i - 1] = x[i - 1] - x[i];
    }
    diff.sort_by(|a, b| b.cmp(a));
    let mut tmp = 0;
    if n >= m {
        println!("{}", 0)
    } else {
        for i in 0..n - 1 {
            tmp += diff[i];
        }
        let res = diff.iter().sum::<i64>() - tmp;
        println!("{}", res);
    }
}
