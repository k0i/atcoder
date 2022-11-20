use proconio::{fastout, input};
use std::collections::HashMap;
#[fastout]
pub fn main() {
    input! {
    H:usize,
    W:usize,
    N:usize,
    h:usize,
    w:usize,
    a:[[usize;W];H]
        }
    let mut set = HashMap::new();
    for i in 0..H {
        for j in 0..W {
            let cur = a[i][j];
            let v = set.entry(cur).or_insert(0);
            *v += 1;
        }
    }
    let mut compress = vec![0; 301];
    for (k, v) in set.iter() {
        compress[*k] = *v;
    }

    let l = set.len();
    let mut cursor_h = 0;

    let mut unq = HashMap::new();
    while cursor_h <= H - h {
        let mut cursor_w = 0;
        while cursor_w <= W - w {
            unq.clear();
            let mut ans = l;
            for i in 0..h {
                for j in 0..w {
                    let cur = a[cursor_h + i][cursor_w + j];
                    let v = unq.entry(cur).or_insert(0);
                    *v += 1;
                    if *v == compress[cur] {
                        ans -= 1;
                    }
                }
            }
            if cursor_w == W - w {
                println!("{}", ans);
            } else {
                print!("{} ", ans);
            }
            cursor_w += 1;
        }
        cursor_h += 1;
    }
}
