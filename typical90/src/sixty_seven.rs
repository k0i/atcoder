use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    mut n:String,
    k:usize
        }

    if n == "0" {
        println!("0");
        return;
    }
    let mut v: Vec<usize> = n
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();
    let mut temp = 0;
    for i in 0..k {
        temp = base8_to_base10(&v);
        v = base10_to_base9(temp);
        let temp = change_five(&v);
        v = temp
            .chars()
            .map(|x| x.to_digit(8).unwrap() as usize)
            .collect();
    }
    println!("{}", v.iter().join(""));
}

fn base8_to_base10(v: &[usize]) -> usize {
    let mut n = 0;
    for i in 0..v.len() {
        n += v[i] * 8usize.pow((v.len() - i - 1) as u32);
    }
    n
}

fn base10_to_base9(n: usize) -> Vec<usize> {
    let mut v = vec![];
    let mut n = n;
    while n > 0 {
        v.push(n % 9);
        n /= 9;
    }
    v.reverse();
    v
}

fn change_five(v: &[usize]) -> String {
    v.iter().map(|x| if *x == 8 { 5 } else { *x }).join("")
}
