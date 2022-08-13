use itertools::Itertools;
use proconio::{
    input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
pub fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [Usize1; n]
    }
    let max = n + 1;
    let mut ord = vec![max; n];

    let mut cnt = 0;
    let mut tmp = 0;
    loop {
        if ord[tmp] == max {
            ord[tmp] = cnt;
        } else {
            k -= ord[tmp];
            k %= cnt - ord[tmp];
            while k > 0 {
                tmp = a[tmp];
                k -= 1;
            }
            println!("{}", tmp + 1);
            return;
        }
        cnt += 1;
        tmp = a[tmp];
        if k == cnt {
            println!("{}", tmp + 1);
            return;
        }
    }
}

fn c() {
    input! {n:usize,m:usize,x:u32,c:[[u32;m+1];n]}
    let mut res = std::i64::MAX;
    for i in 0..(1 << n) {
        let mut books = vec![false; n];
        (0..n)
            .filter(|x| i & 1 << x != 0)
            .for_each(|i| books[i] = true);
        let mut skill = vec![0; m];
        let mut temp: i64 = 0;
        for j in 0..n {
            if books[j] {
                temp += c[j][0] as i64;
                for k in 1..m + 1 {
                    skill[k - 1] += c[j][k];
                }
            }
        }
        if skill.into_iter().filter(|e| e < &x).count() == 0 {
            res = res.min(temp);
        }
    }
    if res == std::i64::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
