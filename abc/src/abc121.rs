#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        mut a:usize,mut b:usize
    }
    let mut v = vec![];
    if a % 2 == 1 {
        v.push(a);
        a += 1;
    }
    if b % 2 == 0 {
        v.push(b);
    } else {
        b += 1;
    }
    if ((b - a) / 2) % 2 != 0 {
        v.push(1);
    }
    println!("{}", v.iter().fold(0, |a, b| a ^ b));
}

fn c() {
    input! {
    n:usize,
    mut m:usize,
    mut a:[(u64,u64);n],
        }

    a.sort_by_key(|x| x.0);
    let mut res = 0;
    let mut temp = 0;
    loop {
        if m == 0 {
            break;
        }
        if a[temp].1 > 0 {
            res += a[temp].0;
            m -= 1;
            a[temp].1 -= 1;
        } else {
            temp += 1;
        }
    }
    println!("{}", res);
}
