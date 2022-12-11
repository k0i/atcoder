use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        w: isize,
        h: isize,
        n: usize,
        xya: [(isize, isize, isize); n],
    };
    let mut left = 0;
    let mut right = w;
    let mut down = 0;
    let mut up = h;
    for (x, y, a) in xya {
        match a {
            1 => left = left.max(x),
            2 => right = right.min(x),
            3 => down = down.max(y),
            4 => up = up.min(y),
            _ => unreachable!(),
        };
    }
    let ans = 0.max(right - left) * 0.max(up - down);
    println!("{}", ans);
}
