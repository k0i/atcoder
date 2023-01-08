use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
        s: Chars,
    }
    let mut check = [false; 26];
    for &c in &s {
        check[(c as u8 - b'a') as usize] = true;
    }
    if s.len() < 26 {
        for i in 0..26 {
            if check[i] {
                continue;
            }
            println!(
                "{}{}",
                s.iter().collect::<String>(),
                (b'a' + i as u8) as char
            );
            return;
        }
    }
    for i in (0..26).rev() {
        let mut min_c = 0;
        for j in ((s[i] as u8 - b'a') + 1) as usize..26 {
            if check[j] {
                continue;
            }
            min_c = j;
            break;
        }
        if min_c == 0 {
            check[(s[i] as u8 - b'a') as usize] = false;
            continue;
        }
        println!(
            "{}{}",
            &s[..i].iter().collect::<String>(),
            (min_c as u8 + b'a') as char
        );
        return;
    }
    println!("-1");
}
