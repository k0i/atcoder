#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    f:[(usize,usize);n],
    q:usize,
    t:[(usize,usize);q],
        }
    let mut first = vec![0; n + 1];
    let mut second = vec![0; n + 1];
    for i in 0..n {
        let (a, b) = f[i];
        if a == 1 {
            first[i + 1] = first[i] + b;
            second[i + 1] = second[i];
        } else {
            first[i + 1] = first[i];
            second[i + 1] = second[i] + b;
        }
    }
    for j in 0..q {
        let (a, b) = t[j];
        println!("{} {}", first[b] - first[a - 1], second[b] - second[a - 1]);
    }
}
