use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    mut p:[usize;n],
        }
    let mut temp = std::usize::MAX;
    let mut ind = n;
    for i in (0..n).rev() {
        if p[i] < temp {
            temp = p[i];
            continue;
        }
        ind = i;
        break;
    }

    let mut max_ind = 0;
    let mut max = 0;
    for i in ind + 1..n {
        if p[ind] > p[i] && p[i] > max {
            max = p[i];
            max_ind = i;
        }
    }
    p.swap(ind, max_ind);
    let mut sorted = p.drain(ind + 1..).collect_vec();
    sorted.sort_by(|a, b| b.cmp(a));
    p.append(&mut sorted);
    for i in 0..n {
        if i == n - 1 {
            println!("{}", p[i]);
        } else {
            print!("{} ", p[i]);
        }
    }
}
