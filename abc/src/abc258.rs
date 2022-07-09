#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
fn main() {
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
