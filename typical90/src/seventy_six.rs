use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {n:usize,mut a:[usize;n]}
    let wanted = a.iter().sum::<usize>() / 10;

    let mut b = a.clone();
    b.append(&mut a);

    let mut cumsum = vec![0];
    for i in b.iter() {
        cumsum.push(cumsum.last().unwrap() + i);
    }

    for i in 0..cumsum.len() {
        let mut left = i;
        let mut right = b.len();
        while right - left > 1 {
            let mid = (left + right) / 2;
            if cumsum[mid] - cumsum[i] >= wanted {
                right = mid;
            } else {
                left = mid;
            }
            if right - left == 1 {
                if cumsum[right] - cumsum[i] == wanted {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
