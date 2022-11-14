use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! { n: usize, a: [usize; n] }

    let mut h = HashMap::new();
    for i in 0..n {
        *h.entry(a[i]).or_insert(0) += 1;
    }
    if h.len() == 1 {
        if let Some(&val) = h.get(&0) {
            if val == n {
                println!("Yes");
                return;
            }
        }
    } else if n % 3 == 0 && h.len() == 2 {
        if let Some(&val) = h.get(&0) {
            if val == n / 3 {
                println!("Yes");
                return;
            }
        }
    } else if n % 3 == 0 && h.len() == 3 {
        let mut v = Vec::new();
        for (&key, &val) in h.iter() {
            if val != n / 3 {
                println!("No");
                return;
            }
            v.push(key);
        }
        if v[0] ^ v[1] ^ v[2] != 0 {
            println!("No");
            return;
        }
        println!("Yes");
        return;
    }
    println!("No");
}
