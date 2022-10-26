use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        mut xy: [(usize, usize); n],
        s: Chars,
    };

    let mut map: HashMap<usize, (usize, usize)> = HashMap::new();

    for i in 0..n {
        let (x, y) = xy[i];
        let c = s[i];

        let map_x = map.entry(y).or_insert((1000000000, 0));

        if c == 'L' {
            map_x.1 = map_x.1.max(x);
        } else {
            map_x.0 = map_x.0.min(x);
        }
    }

    let mut ans = false;

    for (_, (l, r)) in map {
        if l <= r {
            ans = true;
            break;
        }
    }

    println!("{}", if ans { "Yes" } else { "No" });
}
