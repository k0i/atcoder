use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    xy:[(i64,i64);n]
            }
    let mut visited = HashSet::new();
    let mut ans = n;
    for x in 0..n {
        let (i, j) = xy[x];
        visited.insert((i, j));
        for y in x + 1..n {
            let (k, l) = xy[y];
            if k == i && l == j {
                ans -= 1;
            }
            if k == i - 1 && l == j - 1 && !visited.contains(&(i - 1, j - 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
            if k == i - 1 && l == j + 1 && !visited.contains(&(i - 1, j + 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
            if k == i - 1 && l == j - 1 && !visited.contains(&(i - 1, j - 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
            if k == i && l == j + 1 && !visited.contains(&(i, j + 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
            if k == i + 1 && l == j + 1 && !visited.contains(&(i + 1, j + 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
            if k == i + 1 && l == j + 1 && !visited.contains(&(i + 1, j + 1)) {
                ans -= 1;
                visited.insert((k, l));
            }
        }
    }
    println!("{:?}", ans);
}
