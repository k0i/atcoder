use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
        }
    let mut an = vec![];
    let mut res = 0;
    for i in 0..n {
        input! { a: usize,xy:[(usize,usize);a] }
        an.push(xy);
    }

    for i in 0..1 << n {
        let mut honest = vec![false; n];
        let mut ok = true;
        let mut temp = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                honest[j] = true;
            }
        }
        for j in 0..n {
            if honest[j] {
                for k in 0..an[j].len() {
                    let (x, y) = an[j][k];
                    if honest[x - 1] != (y == 1) {
                        ok = false;
                    }
                }
                temp += 1;
            }
        }
        if ok {
            res = std::cmp::max(res, temp)
        }
    }
    println!("{}", res);
}
