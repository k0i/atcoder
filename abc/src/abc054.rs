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
        m: usize,
        a: [Chars; n],
        b: [Chars; m]
    };

    for a_i in 0..=n - m {
        for a_j in 0..=n - m {
            let mut flag = true;
            for b_i in 0..m {
                for b_j in 0..m {
                    flag &= a[a_i + b_i][a_j + b_j] == b[b_i][b_j];
                }
            }
            if flag {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
