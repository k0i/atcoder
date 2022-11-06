use std::collections::HashMap;

use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n:usize,
    s:[Chars;n]
            }

    let mut h = HashMap::new();
    for i in s {
        match i[0] {
            'M' => {
                h.entry("M").and_modify(|e| *e += 1).or_insert(1);
            }
            'A' => {
                h.entry("A").and_modify(|e| *e += 1).or_insert(1);
            }
            'R' => {
                h.entry("R").and_modify(|e| *e += 1).or_insert(1);
            }
            'C' => {
                h.entry("C").and_modify(|e| *e += 1).or_insert(1);
            }
            'H' => {
                h.entry("H").and_modify(|e| *e += 1).or_insert(1);
            }
            _ => {}
        }
    }
    let comb = h.keys().combinations(3).collect::<Vec<_>>();
    let mut ans = 0usize;
    for i in comb {
        ans += h[i[0]] * h[i[1]] * h[i[2]];
    }
    println!("{}", ans);
}
