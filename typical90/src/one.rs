use std::rc::Rc;

use proconio::input;

pub fn main() {
    input! {
    n :usize,
    l:u32,
    k:usize,
    a :[u32;n]
        }
    let mut left = 0;
    let mut right = l;
    let temp = a;

    while right - left > 1 {
        let mid = (left + right) / 2;
        match cut(mid, &temp, l, k) {
            true => left = mid,
            false => right = mid,
        }
    }
    println!("{}", left);
}

fn cut(target: u32, a: &[u32], l: u32, count: usize) -> bool {
    let mut prev = 0;
    let mut cut = 0;
    for i in a.iter() {
        if i - prev >= target && l - i >= target {
            cut += 1;
            prev = *i;
        }
    }
    cut >= count
}
