#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    s:Chars
        }
    let mut white = false;
    let mut pos = "";
    let mut cur = 0;
    'outer: for i in 0..s.len() {
        if s[i] == 'W' {
            if !white {
                white = true
            } else {
                let mut b_count = 0;
                let mut inner_white = false;
                for j in i..s.len() {
                    if s[j] == 'B' {
                        b_count += 1;
                        inner_white = false;
                    } else {
                        if !inner_white {
                            inner_white = true;
                        } else {
                            if b_count == 2 {
                                pos = "Si";
                                cur = i;
                                break 'outer;
                            } else {
                                pos = "Mi";
                                cur = i;
                                break 'outer;
                            }
                        }
                    }
                }
            };
        } else {
            white = false
        }
    }
    while cur / 2 > 0 {
        cur -= 2;
        match pos {
            "Do" => {
                pos = "Si";
                continue;
            }
            "Re" => {
                pos = "Do";
                continue;
            }
            "Mi" => {
                pos = "Re";
                continue;
            }
            "Fa" => {
                pos = "Mi";
                continue;
            }
            "So" => {
                pos = "Fa";
                continue;
            }
            "La" => {
                pos = "So";
                continue;
            }
            "Si" => {
                pos = "La";
                continue;
            }
            _ => {}
        }
    }
    println!("{}", pos);
}
