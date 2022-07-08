#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    w:u64,
    h:u64,
    x:u64,
    y:u64
    }
    let ans1 = (w * h) as f64 / 2.0;
    let mut ans2 = 0;
    if x * 2 == w && y * 2 == h {
        ans2 = 1
    };

    println!("{} {}", ans1, ans2);
}
