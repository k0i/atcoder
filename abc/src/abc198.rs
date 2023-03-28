use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use superslice::Ext;
#[fastout]
pub fn main() {
    d()
}
fn d() {
    input! {
        s: [Bytes; 3],
    }
    let id = |c| (c - b'a') as usize;
    let mut n = 0;
    let mut c2i = vec![9999; 26];
    for t in &s {
        for &c in t {
            if c2i[id(c)] == 9999 {
                c2i[id(c)] = n;
                n += 1;
            }
        }
    }
    if n > 10 {
        println!("UNSOLVABLE");
        return;
    }
    let mut p = (0..10).collect_vec();
    loop {
        let mut f = true;
        let mut x = vec![0i64; 3];
        for i in 0..3 {
            if p[c2i[id(s[i][0])]] == 0 {
                f = false;
                break;
            }
            for &c in &s[i] {
                x[i] = x[i] * 10 + p[c2i[id(c)]];
            }
        }
        if f && x[0] + x[1] == x[2] {
            for i in 0..3 {
                println!("{}", x[i]);
            }
            return;
        }
        if !p.next_permutation() {
            break;
        }
    }
    println!("UNSOLVABLE");
}

fn c() {
    input! {
    r:f64,
    x:f64,y:f64
        }
    let dis = (x * x + y * y).sqrt();
    let ans = (dis / r).ceil() as i64;
    if ans == 1 && dis != r {
        println!("2");
    } else {
        println!("{}", ans);
    }
}
