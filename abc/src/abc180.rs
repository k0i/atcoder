use std::collections::BTreeSet;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
       mut x: usize,
        y: usize,
        a: usize,
        b: usize,
    }

    let mut exp = 0;

    while x * a - x < b {
        if x * a >= y {
            println!("{}", exp);
            return;
        }

        exp += 1;
        x *= a;
    }

    exp += (y - 1 - x) / b;

    println!("{}", exp);
}

fn c() {
    input! {
    n:u64
        }
    let mut div: u64 = 1;
    let mut divs = BTreeSet::new();
    while n >= div.pow(2) {
        if n % div == 0 {
            divs.insert(div);
            if div != n / div {
                divs.insert(n / div);
            }
        }
        div += 1;
    }
    divs.iter().for_each(|x| println!("{}", x));
}
