#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    k:i64,
    a: [i64;n],
    b: [i64;n],
        }
    let distance: i64 = a.iter().zip(b.iter()).map(|(a, b)| (a - b).abs()).sum();
    println!(
        "{}",
        if distance <= k && (k - distance) % 2 == 0 {
            "Yes"
        } else {
            "No"
        }
    );
}
