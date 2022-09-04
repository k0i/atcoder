use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    if s[0] == '1' {
        println!("No");
        return;
    }
    let mut all = vec![];
    if s[6] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[3] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[1] == '0' && s[7] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[0] == '0' && s[4] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[2] == '0' && s[8] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[5] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    if s[9] == '0' {
        all.push(true);
    } else {
        all.push(false)
    }
    for i in 0..all.len() {
        if all[i] == true {
            for j in 0..i {
                if all[j] == false {
                    for k in i + 1..all.len() {
                        if all[k] == false {
                            println!("Yes");
                            return;
                        }
                    }
                }
            }
        }
    }
    println!("No");
}
