use itertools::Itertools;
use proconio::{
    fastout, input,
    marker::{Bytes, Chars, Isize1, Usize1},
};
#[fastout]
pub fn main() {
    input! {
        mut n: usize,
        mut k: usize,
    }
    let max = std::usize::MAX;
    let mut v = vec![max; 100000];

    while v[n] == max {
        let y = sum_digits(n);
        let z = (n + y) % 100000;
        v[n] = z;
        n = z;
        k -= 1;
        if k == 0 {
            println!("{}", n);
            return;
        }
    }

    let first = n;
    let mut cycle_len = 0;
    loop {
        n = v[n];
        k -= 1;

        if k == 0 {
            println!("{}", n);
            return;
        }

        cycle_len += 1;
        if n == first {
            break;
        }
    }

    k %= cycle_len;
    while k > 0 {
        let y = sum_digits(n);
        let z = (n + y) % 100000;
        v[n] = z;
        n = z;
        k -= 1;
    }

    println!("{}", n);
}

fn sum_digits(n: usize) -> usize {
    let mut n = n;
    let mut sum = 0usize;
    while n > 0 {
        let q = n % 10;
        sum += q;
        n /= 10;
    }

    sum
}
