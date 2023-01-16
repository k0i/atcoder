use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect_vec();

    if s.len() == 1 {
        let ans = alphabet.iter().position(|&x| x == s[0]).unwrap();
        println!("{}", ans + 1);
        return;
    }
    let mut ans = 0;
    for i in 0..s.len() - 1 {
        let cur = s[i];
        let cur_pos = alphabet.iter().position(|&x| x == cur).unwrap();
        ans += 26i64.pow((s.len() - i - 1) as u32) * (cur_pos as i64 + 1);
    }
    let cur = s[s.len() - 1];
    let cur_pos = alphabet.iter().position(|&x| x == cur).unwrap();
    ans += cur_pos as i64 + 1;
    println!("{}", ans);
}
