use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    k:u128
    }
    let mut div = vec![];
    let mut i = 1;
    while i * i <= k {
        if k % i == 0 {
            div.push(i);
            if i != k / i {
                div.push(k / i);
            }
        }
        i += 1;
    }
    let mut ans = 0;

    for i in 0..div.len() {
        for j in i..div.len() {
            let a = div[i];
            let b = div[j];
            let m = k % (a * b);
            if m != 0 {
                continue;
            }
            let c = k / (a * b);
            if a <= b && b <= c {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
