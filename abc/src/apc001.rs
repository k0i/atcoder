#![allow(unused_imports)]
use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
    source::line::LineSource,
};
use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufReader},
};
#[fastout]
pub fn main() {
    c()
}

fn c() {
    let mut tab = HashMap::new();
    tab.insert("Vacant".to_string(), 'v');
    tab.insert("Male".to_string(), 'm');
    tab.insert("Female".to_string(), 'f');

    let stdin = stdin();
    let mut source = LineSource::new(BufReader::new(stdin));
    input! { from &mut source, n: usize, };
    let mut v: Vec<char> = vec!['u'; n + 1];
    let mut l = 0;
    let mut r = n;

    println!("0"); // 1st time
    input! { from &mut source, s: String, };
    let c = tab[&s];

    v[l] = c;
    v[r] = c;

    while r - l > 0 {
        let m = (l + r) / 2;
        println!("{}", m);
        input! { from &mut source, s: String, };
        let c = tab[&s];
        v[m] = c;

        if c == 'v' {
            break;
        }

        if (m - l) % 2 == 1 {
            if v[l] == c {
                r = m;
            } else {
                l = m;
            }
        } else {
            if v[l] == c {
                l = m;
            } else {
                r = m;
            }
        }
    }
}

fn b() {
    input! {
        n:usize,
        a:[i64;n],
        b:[i64;n]
    };
    let mut diff = 0;
    for i in 0..n {
        let d = b[i] - a[i];
        diff += if d > 0 { d >> 1 } else { d };
    }
    println!("{}", if diff >= 0 { "Yes" } else { "No" });
}
