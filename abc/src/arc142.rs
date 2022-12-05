use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! { n: usize }
    let mut ans = vec![vec![0; n]; n];
    let mut val = 1;
    for i in 0..n {
        for j in 0..n {
            ans[i][j] = val;
            val += 1;
        }
    }
    for i in 0..n {
        if i % 2 == 1 {
            continue;
        }
        if i + 1 == n {
            continue;
        }

        let mut c = ans[i + 1].clone();
        let mut d = ans[i].clone();
        std::mem::swap(&mut ans[i], &mut c);
        std::mem::swap(&mut ans[i + 1], &mut d);
    }
    for i in 0..n {
        for j in 0..n {
            print!("{} ", ans[i][j]);
        }
        println!();
    }
}
fn a() {
    input! {
     n:usize,
    mut k:Chars
        }
    let mut res = 0;
    let mut a = k.iter().collect::<String>().parse::<usize>().unwrap();
    k.reverse();
    let mut b = k.iter().collect::<String>().parse::<usize>().unwrap();

    if a > b {
        println!("0");
        return;
    }
    let mut dedup = HashSet::new();
    while a <= n {
        if dedup.contains(&a) {
            a *= 10;
            continue;
        }
        res += 1;
        dedup.insert(a);
        a *= 10;
    }
    while b <= n {
        if dedup.contains(&b) {
            b *= 10;
            continue;
        }
        res += 1;
        dedup.insert(b);
        b *= 10;
    }
    println!("{}", res);
}
