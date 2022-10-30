use proconio::{fastout, input};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    h:[usize;n],
        }
    let mut highest = 0;
    for i in 0..n {
        if h[highest] < h[i] {
            highest = i;
        }
    }
    println!("{}", highest + 1);
}
