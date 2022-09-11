use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,w:usize,n:usize,a:[usize;n]
        }
    let mut res = vec![vec![0; w]; h];
    let mut pos = (0, 0);
    let mut rev = false;
    for i in 0..n {
        let mut clr = a[i];
        'inner: loop {
            if clr == 0 {
                break 'inner;
            }
            res[pos.0][pos.1] = i + 1;
            if rev {
                if pos.1 == 0 {
                    pos.0 += 1;
                    rev = false;
                } else {
                    pos.1 -= 1;
                }
            } else {
                if pos.1 == w - 1 {
                    pos.0 += 1;
                    rev = true;
                } else {
                    pos.1 += 1;
                }
            }
            if pos.0 == h {
                break;
            }
            clr -= 1;
        }
    }
    for i in res {
        for j in i {
            print!("{} ", j)
        }
        println!();
    }
}
