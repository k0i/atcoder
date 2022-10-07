use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let mut abc = vec![0; 3];
    for i in s {
        match i {
            'x' => abc[0] += 1,
            'o' => abc[1] += 1,
            _ => abc[2] += 1,
        }
    }
    if abc[1] > 4 || abc[1] + abc[2] == 0 {
        println!("0");
        return;
    }
    let mut res = 0;
    for i in 0..=abc[2] {
        if abc[1] + i == 0 || abc[1] + i > 4 {
            continue;
        }
        let cnt = abc[1] + i;
        if cnt == 1 {
            res += 1 * acb(abc[2], i)
        } else if cnt == 2 {
            res += (2 * 4 + acb(4, 2)) * acb(abc[2], i)
        } else if cnt == 3 {
            res += 3 * 4 * 3 * acb(abc[2], i)
        } else if cnt == 4 {
            res += 4 * 3 * 2 * acb(abc[2], i)
        };
    }
    println!("{}", res);
}

fn acb(a: usize, b: usize) -> usize {
    let mut res = 1;
    for i in 0..b {
        res *= a - i;
    }
    for i in 1..=b {
        res /= i;
    }
    res
}
