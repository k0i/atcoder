use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};

#[fastout]
pub fn main() {
    input! {
        mut k: u64,
    }
    let mut digit = 0;
    while k > 0 {
        for i in 1..10 {
            k = lunlun(k, digit, i);
        }
        digit += 1;
    }
}

fn lunlun(k: u64, digit: u64, current: u64) -> u64 {
    if k == 0 {
        return k;
    }
    let mut k = k;
    if digit == 0 {
        k -= 1;
        if k == 0 {
            println!("{}", current);
        }
        return k;
    }
    let prev = current % 10;
    let start = if prev == 0 { 0 } else { prev - 1 };
    let end = if prev == 9 { 9 } else { prev + 1 };
    for d in start..=end {
        k = lunlun(k, digit - 1, current * 10 + d);
    }
    k
}
