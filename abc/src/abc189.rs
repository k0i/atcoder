#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut x: (i64, i64) = (1, 1);

    for i in 0..s.len() {
        if s[i] == "AND" {
            x.1 = x.1 * 2 + x.0;
        } else {
            x.0 = x.0 * 2 + x.1;
        }
    }
    println!("{}", x.0);
}

fn c() {
    input! {
        n:usize,
        a:[u64;n]
    }

    let mut ans = 0;
    for l in 0..n {
        let mut tmp = a[l];
        for r in l..n {
            tmp = tmp.min(a[r]);
            let tmp_sum = tmp * ((r - l + 1) as u64);
            ans = ans.max(tmp_sum);
        }
    }
    println!("{}", ans);
}

fn b() {
    input! {n:usize,mut x:u64,c:[(u64,u64);n]}
    x *= 100;
    let mut sum = 0;
    let mut res = -1;
    for i in 0..n {
        let (v, p) = c[i];
        sum += v * p;
        if sum > x {
            res = (i + 1) as i64;
            break;
        }
    }
    println!("{}", res);
}
