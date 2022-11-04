use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[[u64;n];n],
    m:usize,
    xy:[(Usize1,Usize1);m],
        }
    let mut ng = vec![vec![false; n]; n];
    let perm: Vec<_> = (0..n).permutations(n).collect();
    for i in 0..m {
        let (x, y) = xy[i];
        ng[x][y] = true;
        ng[y][x] = true;
    }
    let mut res = std::u64::MAX;
    'outer: for i in 0..perm.len() {
        let mut temp = 0;
        let p = &perm[i];
        for j in 0..p.len() {
            let ai = a[p[j]][j];
            if j > 0 && ng[p[j]][p[j - 1]] {
                continue 'outer;
            }
            temp += ai;
        }
        res = std::cmp::min(res, temp);
    }
    if res == std::u64::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
