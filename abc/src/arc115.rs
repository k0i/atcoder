use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        n: usize,
        c: [[usize;n]; n],
    }

    let mut min = 1000000001;
    let mut min_a_index = 0;
    for i in 0..n {
        let m = c[i].iter().min().unwrap();
        if min > *m {
            min = *m;
            min_a_index = i;
        }
    }

    let mut b: Vec<usize> = vec![0; n];
    for i in 0..n {
        b[i] = c[min_a_index][i];
    }

    let mut a: Vec<usize> = vec![0; n];
    for i in 0..n {
        if c[i][0] < b[0] {
            println!("No");
            return;
        }
        a[i] = c[i][0] - b[0];
    }

    for i in 0..n {
        for j in 0..n {
            if a[i] + b[j] != c[i][j] {
                println!("No");
                return;
            }
        }
    }

    let a: Vec<_> = a.iter().map(|&v| format!("{}", v)).collect();
    let b: Vec<_> = b.iter().map(|&v| format!("{}", v)).collect();

    println!("Yes");
    println!("{} ", a.join(" "));
    println!("{} ", b.join(" "));
}

fn a() {
    input! {
        n: usize,
        _: usize,
        s: [Chars; n]
    };
    let mut e = 0usize;
    let mut o = 0usize;
    for s_i in s {
        if s_i.iter().copied().filter(|&s_ij| s_ij == '1').count() % 2 == 0 {
            e += 1;
        } else {
            o += 1;
        }
    }
    println!("{}", e * o);
}
