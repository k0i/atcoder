#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
use std::collections::VecDeque;
#[fastout]
pub fn main() {
    d()
}
pub fn d() {
    input! {q:usize}
    let mut v = VecDeque::new();
    for _ in 0..q {
        input! {k:usize}
        if k == 1 {
            input! {x:usize,c:usize}
            v.push_back((x, c));
        } else {
            input! {mut c:usize}
            let mut ans = 0;
            while let Some((x, num)) = v.pop_front() {
                if c >= num {
                    ans += x * num;
                    c -= num;
                } else {
                    ans += x * c;
                    v.push_front((x, num - c));
                    break;
                }
            }
            println!("{}", ans);
        }
    }
}

pub fn c() {
    input! {n: usize}
    let ans = recurse(n);
    for i in ans {
        print!("{} ", i);
    }
}

fn recurse(n: usize) -> Vec<usize> {
    if n == 1 {
        vec![n]
    } else {
        let mut new = recurse(n - 1);
        new.push(n);
        new.append(&mut recurse(n - 1));
        new
    }
}
pub fn b() {
    input! {n:usize,
        a:[(String,String);n],
    }
    let mut ans = "Yes";
    for i in 0..n {
        let mut f = false;
        let mut na = false;
        let (family, name) = &a[i];
        for j in 0..n {
            if i == j {
                continue;
            }
            let (c, d) = &a[j];

            if family == c {
                f = true;
            }
            if family == d {
                f = true;
            }
            if name == c {
                na = true;
            }
            if name == d {
                na = true;
            }
        }
        if f && na {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}

pub fn a() {
    input! {s: Chars}
    for i in 0..s.len() {
        if i == 0 {
            print!("{}", 0);
            continue;
        }
        print!("{}", s[i - 1]);
    }
}
