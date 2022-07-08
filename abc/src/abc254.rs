#[allow(unused_imports)]
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {}

fn c() {
    input! {n:usize,k:usize,mut a:[u64;n]}
    let mut sorted = vec![vec![]; k];

    for i in 0..n {
        let idx = i % k;
        sorted[idx].push(a[i]);
    }
    for i in 0..k {
        sorted[i].sort();
    }
    let mut b = a;
    b.sort();
    for i in 0..n {
        let idx = i % k;
        if b[i] != sorted[idx][i / k] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

fn b() {
    input! {n:usize}
    let mut res = vec![vec![]];
    for i in 0..n {
        if i == 0 {
            res[0].push(1);
            continue;
        }
        let mut tmp = vec![];
        for j in 0..i + 1 {
            if j == 0 || j == i {
                tmp.push(1);
            } else {
                tmp.push(res[i - 1][j - 1] + res[i - 1][j]);
            }
        }
        res.push(tmp);
    }
    for i in res {
        for j in i {
            print!("{} ", j);
        }
        println!();
    }
}
fn a() {
    input! {
    n:u64
        }
    let a = n.to_string().chars().rev().take(2).collect::<String>();
    let b = a.to_string().chars().rev().collect::<String>();
    println!("{}", b);
}
