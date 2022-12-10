use itertools::Itertools;
#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    a:u128,
    b:f64,
       }
    let b = (b * 100.0).round() as u128;
    println!("{:?}", b);
    let ans = a * b / 100;
    println!("{}", ans);
}

fn d() {
    input! {mut n:usize}
    let mut p = vec![];
    if n == 1 {
        println!("0");
        return;
    }
    let mut i = 2;
    let mut a = n;
    while i * i <= a {
        if a % i == 0 {
            p.push(i);
            a /= i;
        } else {
            i += 1;
        }
    }
    if a != 1 {
        p.push(a);
    }
    p.dedup();
    let mut res = 0;
    'outer: for i in p.iter() {
        for j in 1..100 {
            if n % i.pow(j) == 0 {
                n /= i.pow(j);
                res += 1;
            } else {
                continue 'outer;
            }
        }
    }
    println!("{:?}", res);
}

fn b() {
    input! {
    n:usize,
    a:[u128;n],
        }
    let mut ans = 1;
    let mut overflow = false;
    let mut zero = false;
    for i in 0..n {
        ans *= a[i];
        if ans > 10_u128.pow(18) {
            overflow = true;
        }
        if a[i] == 0 {
            zero = true;
        }
    }
    if zero {
        println!("0");
    } else if overflow {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
