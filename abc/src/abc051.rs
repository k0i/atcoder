use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
      sx: isize, sy: isize, tx: isize, ty: isize
    }

    let dx = (tx - sx) as usize;
    let dy = (ty - sy) as usize;
    let path1 = "R".repeat(dx) + &"U".repeat(dy);
    let path2 = "L".repeat(dx) + &"D".repeat(dy);
    let path3 = "D".to_owned() + &"R".repeat(dx + 1) + &"U".repeat(dy + 1) + "L";
    let path4 = "U".to_owned() + &"L".repeat(dx + 1) + &"D".repeat(dy + 1) + "R";

    println!("{}", path1 + &path2 + &path3 + &path4);
}
fn b() {
    input! {
    k:usize,
    s:usize
        }

    let mut ans = 0;
    for i in 0..=k {
        for j in 0..=k {
            if i + j <= s && s - i - j <= k {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
