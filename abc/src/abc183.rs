use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
    n:usize,
    w:isize,
    stp:[(isize,isize,isize);n]
    }
    let mut imos = vec![0; 2 * 100000 + 1];
    for i in 0..n {
        let (start, end, amount) = stp[i];
        imos[start as usize] += amount;
        imos[end as usize] -= amount;
    }
    let mut ans = "Yes";
    for i in 0..2 * 10usize.pow(5) {
        imos[i + 1] += imos[i];
        if imos[i] > w {
            ans = "No";
            break;
        }
    }
    println!("{}", ans);
}

fn c() {
    input! {n:usize,k:u64,t:[[u64;n];n]}
    let mut res = 0;
    for p in (0..n).permutations(n) {
        if p[0] != 0 {
            continue;
        }
        let mut sum = 0;
        for i in 1..n {
            sum += t[p[i - 1]][p[i]];
        }
        sum += t[p[n - 1]][p[0]];
        if sum == k {
            res += 1;
        }
    }
    println!("{}", res);
}
