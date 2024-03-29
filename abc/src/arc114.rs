use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
pub fn main() {
    input! {
        n: usize,
        fs: [usize; n],
    }

    let mut result = 1;

    let mut checked: Vec<usize> = vec![0; n];

    for i in 0..n {
        if checked[i] != 0 {
            continue;
        }

        let mut j = i;

        while checked[j] == 0 {
            checked[j] = i + 1;
            j = fs[j] - 1;
        }

        if checked[j] == i + 1 {
            result *= 2;
            result %= 998244353;
        }
    }

    result += 998244353;
    result -= 1;
    result %= 998244353;
    println!("{}", result);
}

fn a() {
    input! {
    n:usize,
    x:[u64;n]
    }

    let v = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];
    let mut ans = std::u64::MAX;
    for i in 0..(1 << v.len()) {
        let mut t = 1u64;
        for j in 0..v.len() {
            if i >> j & 1 == 1 {
                t *= v[j];
            }
        }
        let mut ok = true;
        for j in &x {
            if gcd(*j, t) == 1 {
                ok = false;
                break;
            }
        }
        if ok {
            ans = ans.min(t);
        }
    }
    println!("{}", ans);
}

fn gcd(a: u64, b: u64) -> u64 {
    return if b == 0 { a } else { gcd(b, a % b) };
}
