use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    k:usize,
    c:[Chars;h]
        }
    let mut res = 0;
    for i in 0..1 << h {
        let mut cc = c.clone();
        for j in 0..1 << w {
            let mut bl = 0;
            for t in 0..h {
                for y in 0..w {
                    if i >> t & 1 == 0 && j >> y & 1 == 0 && c[t][y] == '#' {
                        bl += 1;
                    }
                }
            }
            if bl == k {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
