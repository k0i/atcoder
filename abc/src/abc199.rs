use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,mut s:Chars,q:usize
        }
    let mut rev = false;
    for i in 0..q {
        input! {
        tab:usize,a:usize,b:usize
        }
        if tab == 1 {
            if rev {
                if a <= n && b <= n {
                    s.swap(a + n - 1, b + n - 1);
                } else if a <= n && b > n {
                    s.swap(a + n - 1, b - n - 1);
                } else if a > n && b <= n {
                    s.swap(a - n - 1, b + n - 1);
                } else {
                    s.swap(a - n - 1, b - n - 1);
                }
            } else {
                s.swap(a - 1, b - 1);
            }
        } else {
            rev = !rev;
        }
    }
    if rev {
        let (a, b) = s.split_at(n);
        print!("{}", b.iter().join(""));
        println!("{}", a.iter().join(""));
    } else {
        println!("{}", s.iter().join(""));
    }
}
