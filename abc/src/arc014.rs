use std::collections::HashSet;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:usize,
        }
    let mut a = vec![];
    for i in 0..n {
        input! {
        s:String
        }
        a.push(s);
    }

    let mut tk = HashSet::new();
    let mut tk2 = HashSet::new();
    tk.insert(a[0].clone());
    for i in 1..n {
        if i % 2 == 0 {
            if tk.contains(&a[i]) || tk2.contains(&a[i]) {
                println!("LOSE");
                return;
            }
            if a[i].chars().nth(0).unwrap() != a[i - 1].chars().nth(a[i - 1].len() - 1).unwrap() {
                println!("LOSE");
                return;
            }
            tk.insert(a[i].clone());
            continue;
        }

        if tk2.contains(&a[i]) || tk.contains(&a[i]) {
            println!("WIN");
            return;
        }
        if a[i].chars().nth(0).unwrap() != a[i - 1].chars().nth(a[i - 1].len() - 1).unwrap() {
            println!("WIN");
            return;
        }
        tk2.insert(a[i].clone());
    }
    println!("DRAW");
}
