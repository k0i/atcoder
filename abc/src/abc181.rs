#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {n:usize,p:[(i64,i64);n]}
    let mut res = "No";
    'outer: for i in 0..n {
        for j in i + 1..n {
            for k in j + 1..n {
                if (p[k].1 - p[i].1) * (p[j].0 - p[i].0) == (p[j].1 - p[i].1) * (p[k].0 - p[i].0) {
                    res = "Yes";
                    break 'outer;
                }
            }
        }
    }
    println!("{}", res);
}
