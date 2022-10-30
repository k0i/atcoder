use itertools::Itertools;
use proconio::{fastout, input, marker::Chars};
#[fastout]
pub fn main() {
    input! {
    s:[Chars;9]
        }
    let mut coor = vec![];
    for i in 0..9 {
        for j in 0..9 {
            let a = s[i][j];
            if a == '.' {
                continue;
            }
            coor.push((i as i64, j as i64));
        }
    }
    let comb: Vec<_> = coor.iter().combinations(4).collect();
    let mut res = 0;
    for i in 0..comb.len() {
        let (a, b, c, d) = (comb[i][0], comb[i][1], comb[i][2], comb[i][3]);
        // 円順列6通り
        if check(a, b, c, d)
            || check(a, b, d, c)
            || check(a, c, b, d)
            || check(a, c, d, b)
            || check(a, d, b, c)
            || check(a, d, c, b)
        {
            res += 1;
        }
    }
    println!("{}", res);
}

fn check(a: &(i64, i64), b: &(i64, i64), c: &(i64, i64), d: &(i64, i64)) -> bool {
    let dx = (a.0 - b.0).abs();
    let dy = (a.1 - b.1).abs();
    if b.0 + dy == c.0
        && b.1 + dx == c.1
        && c.0 - dx == d.0
        && c.1 + dy == d.1
        && d.0 - dy == a.0
        && d.1 - dx == a.1
    {
        return true;
    }
    false
}
