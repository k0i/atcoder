use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    q:usize,
    mut a:[usize;n],
        }
    let mut shift = 0;
    for i in 0..q {
        input! {t:usize,mut x:i32,mut y:i32}
        match t {
            1 => {
                x -= shift + 1;
                y -= shift + 1;
                while x < 0 {
                    x += n as i32;
                }
                while y < 0 {
                    y += n as i32;
                }
                a.swap(x as usize, y as usize);
            }
            2 => {
                shift += 1;
            }
            _ => {
                x -= 1;
                x -= shift;
                if x < 0 {
                    x += n as i32;
                }
                println!("{}", a[x as usize]);
            }
        }
    }
}
