use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
    n:usize,
    x:u128,
        }

    let mut num = vec![1];

    for i in 0..n {
        input! {l:usize, ans:[u128;l]}
        let mut temp = vec![];
        for j in num.clone().iter() {
            if *j > x {
                continue;
            }
            for k in ans.iter() {
                temp.push(j * k);
            }
        }
        num = temp;
    }
    println!("{}", num.into_iter().filter(|&r| r == x).count());
}
