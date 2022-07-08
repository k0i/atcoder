#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
static M: usize = 9999;
pub fn main() {
    input! {
    n:i64,
    a:i64,
    b:i64,
    c:i64
    }
    let mut ans = std::usize::MAX;
    'outer: for i in 0..=M {
        for j in 0..=M - i {
            if b * j as i64 > n {
                continue 'outer;
            }
            let k = (n - (a * i as i64) - (b * j as i64)) / c;
            if k < 0 {
                continue 'outer;
            }
            if a * i as i64 + b * j as i64 + c * k as i64 == n {
                if ans > i + j + k as usize {
                    ans = i + j + k as usize;
                }
            }
        }
    }
    println!("{}", ans);
}
