use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    d:i64,
    mut lr:[(i64,i64);n],
    }

    lr.sort_by_key(|x| x.1);

    let mut cu = -1000000000000000000;
    let mut res = 0;
    for i in 0..n {
        let (l, r) = lr[i];
        if cu + d - 1 < l {
            res += 1;
            cu = r;
        }
    }
    println!("{}", res);
}

fn c() {
    input! {
        _: usize,
        a: i64,
        b: i64,
        p: usize,
        q: usize,
        r: usize,
        s: usize,
    }
    let mut ans = vec![vec!['.'; s - r + 1]; q - p + 1];

    for (i, line) in ans.iter_mut().enumerate() {
        for (j, ch) in line.iter_mut().enumerate() {
            let ii = (i + p) as i64;
            let jj = (j + r) as i64;
            // println!("{} {}", ii, jj);
            if ii - jj == a - b || ii + jj == a + b {
                *ch = '#';
            }
        }
    }

    for line in ans {
        for ch in line {
            print!("{}", ch);
        }
        println!();
    }
}
