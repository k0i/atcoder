use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {a:usize,b:usize}
    let mut res = 0;
    let point = vec![1, 2, 4];
    let mut flag = vec![false; 3];
    if a == 1 {
        flag[0] = true;
    } else if a == 2 {
        flag[1] = true;
    } else if a == 4 {
        flag[2] = true;
    } else if a == 3 {
        flag[0] = true;
        flag[1] = true;
    } else if a == 5 {
        flag[0] = true;
        flag[2] = true;
    } else if a == 6 {
        flag[1] = true;
        flag[2] = true;
    } else if a == 7 {
        flag[0] = true;
        flag[1] = true;
        flag[2] = true;
    }
    if b == 1 {
        flag[0] = true;
    } else if b == 2 {
        flag[1] = true;
    } else if b == 4 {
        flag[2] = true;
    } else if b == 3 {
        flag[0] = true;
        flag[1] = true;
    } else if b == 5 {
        flag[0] = true;
        flag[2] = true;
    } else if b == 6 {
        flag[1] = true;
        flag[2] = true;
    } else if b == 7 {
        flag[0] = true;
        flag[1] = true;
        flag[2] = true;
    }
    for i in 0..3 {
        if flag[i] {
            res += point[i];
        }
    }

    println!("{}", res);
}
