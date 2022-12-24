use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{BTreeSet, HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    mut h:usize,
    mut w:usize,
        }
    let mut an = vec![];
    an.push(vec!['#'; w + 2]);
    for _ in 0..h {
        let mut temp = vec!['#'];
        for _ in 0..w {
            input! {
            a:char
                }
            temp.push(a);
        }
        temp.push('#');
        an.push(temp);
    }
    an.push(vec!['#'; w + 2]);

    h += 2;
    w += 2;
    let mut isolated = BTreeSet::new();
    let dx = vec![-1, 0, 1, 0];
    let dy = vec![0, -1, 0, 1];
    for i in 0..h {
        'w: for j in 0..w {
            let cur = an[i][j];
            if cur == '#' {
                continue;
            }
            for k in 0..4 {
                let nx = i as i64 + dx[k];
                let ny = j as i64 + dy[k];
                if cur == an[nx as usize][ny as usize] {
                    continue 'w;
                }
            }
            isolated.insert((i, j));
        }
    }
    let mut ans = -1;
    let mut i = 0;
    while !isolated.is_empty() {
        let (x, y) = isolated.iter().next().unwrap().clone();
        if isolated.contains(&(x - 1, y)) {
            isolated.remove(&(x - 1, y));
            isolated.remove(&(x, y));
            ans += 1;
            continue;
        }
        if isolated.contains(&(x + 1, y)) {
            isolated.remove(&(x + 1, y));
            isolated.remove(&(x, y));
            ans += 1;
            continue;
        }
    }
    println!("{:?}", isolated);
    println!("{}", ans);
}
