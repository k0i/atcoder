use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        n: usize,
    };
    f(&mut String::new(), n, b'a');
}
fn f(s: &mut String, n: usize, c: u8) {
    if s.len() == n {
        println!("{}", s);
        return;
    }
    for i in b'a'..=c {
        s.push(i as char);
        f(s, n, c + if i == c { 1 } else { 0 });
        s.pop();
    }
}

fn c() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let d = c - a - b;
    if d > 0 && 4 * a * b < d * d {
        println!("Yes");
    } else {
        println!("No");
    }
}
