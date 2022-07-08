use std::collections::BTreeMap;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:u64,
    a:u64,
    b:u64
        }
    let am = n / a;
    let bm = n / b;
    let lcm = a * b / gcd(a, b);
    let lcm_m = n / lcm;
    let sum = n * (n + 1) / 2;
    let a_sum = (a + (am * a)) * am / 2;
    let b_sum = (b + (bm * b)) * bm / 2;
    let lcm_sum = (lcm + (lcm_m * lcm)) * lcm_m / 2;
    println!("{}", sum - a_sum - b_sum + lcm_sum);
    println!("{}", lcm);
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[fastout]
fn c() {
    input! {
    n:usize
        }
    let mut h = BTreeMap::new();
    for _ in 0..n {
        input! {a:usize}
        match a {
            1 => {
                input! {x:usize}
                let v = h.entry(x).or_insert(0);
                *v += 1;
            }
            2 => {
                input! {x:usize,y:usize}
                let v = h.entry(x).or_insert(0);
                *v -= std::cmp::min(*v, y);
                if *v == 0 {
                    h.remove(&x);
                }
            }
            _ => {
                println!(
                    "{}",
                    h.iter().last().unwrap().0 - h.iter().next().unwrap().0
                );
            }
        }
    }
}
