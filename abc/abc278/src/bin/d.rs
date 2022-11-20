use std::collections::HashMap;

use proconio::{fastout, input, marker::Usize1};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    a:[usize;n],
    q:usize
        }
    let mut q1 = -1;
    let mut q2 = HashMap::new();
    for _ in 0..q {
        input! {
        i:usize
        }
        match i {
            1 => {
                input! {s:i64}
                q1 = s;
                q2.clear();
            }
            2 => {
                input! {
                k:Usize1,
                x:usize
                }
                let v = q2.entry(k).or_insert(0);
                *v += x;
            }
            _ => {
                input! {r:Usize1}
                let mut l = a[r];
                if q1 != -1 {
                    l = q1 as usize;
                }
                if let Some(x) = q2.get(&r) {
                    l += x;
                }
                println!("{}", l);
            }
        }
    }
}
