use std::collections::BTreeSet;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
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
