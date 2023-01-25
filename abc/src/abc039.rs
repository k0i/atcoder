use std::collections::HashSet;

#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    h:usize,
    w:usize,
    mut s:[Chars;h]
    }
    let mut original = s.clone();
    let mut white_like = HashSet::new();
    let mut dx = vec![-1, -1, -1, 0, 0, 1, 1, 1];
    let mut dy = vec![-1, 0, 1, -1, 1, -1, 0, 1];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            white_like.insert((i, j));
            for k in 0..8 {
                let nx = i as isize + dx[k];
                let ny = j as isize + dy[k];
                if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                    continue;
                }
                white_like.insert((nx as usize, ny as usize));
            }
        }
    }
    for (i, j) in white_like {
        s[i][j] = '.';
    }
    let after = s.clone();

    let mut black_like = HashSet::new();
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                black_like.insert((i, j));
                for k in 0..8 {
                    let nx = i as isize + dx[k];
                    let ny = j as isize + dy[k];
                    if nx < 0 || nx >= h as isize || ny < 0 || ny >= w as isize {
                        continue;
                    }
                    black_like.insert((nx as usize, ny as usize));
                }
            }
        }
    }
    for (i, j) in black_like {
        s[i][j] = '#';
    }
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != original[i][j] {
                println!("impossible");
                return;
            }
        }
    }
    println!("possible");
    for i in 0..h {
        for j in 0..w {
            if after[i][j] == '#' {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}
fn c() {
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
