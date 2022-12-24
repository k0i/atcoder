use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::{HashMap, HashSet};
#[fastout]
pub fn main() {
    input! {
    mut s:Chars
        }

    let mut zero = false;
    let mut ans = 0;
    for i in s {
        match i {
            '0' => {
                if zero {
                    ans += 1;
                    zero = false;
                    continue;
                }
                zero = true;
            }
            _ => {
                if zero {
                    zero = false;
                    ans += 1;
                }
                ans += 1;
            }
        }
    }
    if zero {
        ans += 1;
    }
    println!("{}", ans);
}
