#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:Chars}
    let m: Vec<u64> = n.iter().map(|x| x.to_digit(10).unwrap() as u64).collect();
    let sum: u64 = m.iter().sum();
    if sum % 3 == 0 {
        println!("0");
        return;
    }
    let mut modulo_one = 0;
    let mut modulo_two = 0;
    for i in m.clone() {
        if i % 3 == 1 {
            modulo_one += 1;
        } else if i % 3 == 2 {
            modulo_two += 1;
        }
    }
    if sum % 3 == 1 {
        if modulo_one >= 1 && m.len() > 1 {
            println!("1");
        } else if modulo_two >= 2 && m.len() > 2 {
            println!("2");
        } else {
            println!("-1");
        }
        return;
    }
    if modulo_two >= 1 && m.len() > 1 {
        println!("1");
    } else if modulo_one >= 2 && m.len() > 2 {
        println!("2");
    } else {
        println!("-1");
    }
}
