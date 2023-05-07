use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    d()
}

fn d() {
    input! {
        n: usize,
        a: [u64; n],
    };
    let mut zeros: Vec<u64> = vec![0; 61];
    let mut ones: Vec<u64> = vec![0; 61];

    for i in 0..n {
        for j in 0..61 {
            if a[i] & (1 << j) == 0 {
                zeros[j] += 1;
            } else {
                ones[j] += 1;
            }
        }
    }

    const MOD: u64 = 1000000007;
    let mut res = 0;
    for i in 0..61 {
        let scale = (1_u64 << i) % MOD;
        let mut r = (ones[i] * scale) % MOD;
        r = (zeros[i] * r) % MOD;
        res = (res + r) % MOD;
    }
    println!("{}", res);
}

fn c() {
    input! {
    n:usize,
        }
    let mut an = vec![];
    let mut res = 0;
    for i in 0..n {
        input! { a: usize,xy:[(usize,usize);a] }
        an.push(xy);
    }

    for i in 0..1 << n {
        let mut honest = vec![false; n];
        let mut ok = true;
        let mut temp = 0;
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                honest[j] = true;
            }
        }
        for j in 0..n {
            if honest[j] {
                for k in 0..an[j].len() {
                    let (x, y) = an[j][k];
                    if honest[x - 1] != (y == 1) {
                        ok = false;
                    }
                }
                temp += 1;
            }
        }
        if ok {
            res = std::cmp::max(res, temp)
        }
    }
    println!("{}", res);
}
