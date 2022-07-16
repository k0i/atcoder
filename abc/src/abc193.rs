use std::collections::HashSet;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:u64}
    let mut a: u64 = 2;
    let mut b: u64 = 2;
    let mut res = n;
    let mut decre = HashSet::new();
    while a.pow(2) <= n {
        while a.pow(b as u32) <= n {
            if decre.contains(&a.pow(b as u32)) {
                b += 1;
                continue;
            };
            decre.insert(a.pow(b as u32));
            res -= 1;
            b += 1;
        }
        a += 1;
        b = 2;
    }
    println!("{:?}", res);
}
