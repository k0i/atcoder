#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    c()
}
fn c() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        ql: [(usize, usize); q]
    }
    let mut rot = 0;
    for (qi, v) in ql {
        if qi == 1 {
            rot += v;
            rot %= n;
        } else {
            let ind = (v - 1 + n - rot) % n;
            println!("{}", s[ind]);
        }
    }
}

fn d() {
    input! {n:usize,x:usize,r:[(u64,u64);n]}
    let mut min = std::u64::MAX;
    let mut sum = 0;
    for i in 0..n {
        if i > 0 {
            sum += r[i - 1].0 + r[i - 1].1;
        }
        let mut l = sum;
        l += r[i].0 + r[i].1;
        l += (x - i - 1) as u64 * r[i].1;
        min = std::cmp::min(min, l);
    }
    println!("{}", min);
}

fn b() {
    input! { n: usize, a: [Chars; n] }

    let di = vec![0, 0, 1, n - 1, 1, n - 1, n - 1, 1];
    let dj = vec![1, n - 1, 0, 0, 1, n - 1, 1, n - 1];

    let mut ans = String::new();
    for i in 0..n {
        for j in 0..n {
            for d in 0..8 {
                let mut s = String::new();
                for k in 0..n {
                    s.push(a[(i + di[d] * k) % n][(j + dj[d] * k) % n]);
                }
                ans = ans.max(s);
            }
        }
    }
    println!("{}", ans);
}
